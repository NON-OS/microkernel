// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! The replies a lookup waits for, as the pump delivers them.

use crate::cell::RELAY_BEGIN;
use crate::circuit::Hop;
use crate::onion::cells::{
    begin_onion, introduce_ack, rendezvous2, RELAY_INTRODUCE_ACK, RELAY_RENDEZVOUS2,
    RELAY_RENDEZVOUS_ESTABLISHED,
};
use crate::onion::hs_ntor::rend_keys;
use crate::protocol::HOPS;
use crate::trace;

use super::super::out::send_relay;
use super::super::state::Manager;
use super::circuits::drop_circuit;
use super::job::{Outcome, Stage};

/// The relay at the end of the circuit, the one these replies come from.
const LAST_RELAY: usize = HOPS - 1;

/// `command` with `body` arrived from hop `hop` of the circuit at `index`.
/// Anything not from the last relay of a circuit a lookup is waiting on is
/// ignored, as a reply nobody asked for.
pub fn inbound(state: &mut Manager, index: usize, hop: usize, command: u8, body: &[u8]) {
    if hop != LAST_RELAY {
        return;
    }
    let circuit = state.circuits[index].id;
    let Some(at) = state.onion.iter().position(|j| j.rend == Some(circuit) || j.circuit == Some(circuit)) else {
        return;
    };
    let on_rend = state.onion[at].rend == Some(circuit);
    match (command, on_rend, state.onion[at].stage) {
        (RELAY_RENDEZVOUS_ESTABLISHED, true, Stage::Rendezvous) if state.onion[at].sent => {
            let job = &mut state.onion[at];
            job.stage = Stage::Introduce;
            job.sent = false;
            job.rend_ready = true;
            trace::say_num(b"onion rendezvous point established", circuit as u64);
        }
        (RELAY_INTRODUCE_ACK, false, Stage::Introduce) if state.onion[at].sent => acked(state, at, body),
        (RELAY_RENDEZVOUS2, true, Stage::Introduce | Stage::Waiting) => met(state, at, index, body),
        _ => {}
    }
}

fn acked(state: &mut Manager, at: usize, body: &[u8]) {
    let status = introduce_ack(body);
    let job = &mut state.onion[at];
    let Some(id) = job.circuit.take() else { return };
    if status == Some(0) {
        /* The introduction point passed it on. Its circuit has done its
         * work; the service now comes to the rendezvous point. */
        job.stage = Stage::Waiting;
        /* Zero tells the next tick to start the service's clock: the pump
         * that delivered this carries no time. */
        job.step_deadline = 0;
        trace::say(b"onion introduction accepted");
    } else {
        /* Refused: the next introduction point is tried at the next tick,
         * and a puzzle for it is solved at a higher effort. */
        job.unreachable = job.unreachable.saturating_add(1);
        job.ephemeral = None;
        job.introduced = None;
        job.sent = false;
        trace::say_num(b"onion introduction refused, status", status.unwrap_or(u16::MAX) as u64);
    }
    drop_circuit(state, id);
}

/// RENDEZVOUS2: finish hs-ntor, add the service as the fourth hop and send
/// BEGIN for the caller's stream on it.
fn met(state: &mut Manager, at: usize, index: usize, body: &[u8]) {
    let job = &mut state.onion[at];
    let (Some((server, server_auth)), Some(ephemeral), Some(point)) =
        (rendezvous2(body), job.ephemeral.as_ref(), job.introduced.as_ref())
    else {
        return;
    };
    let (Ok(dh_yx), Ok(dh_bx)) = (ephemeral.shared(&server), ephemeral.shared(&point.enc_key)) else {
        job.outcome = Some(Outcome::Failed(super::directory::REASON_RESOLVEFAILED));
        return;
    };
    let Some(mut keys) = rend_keys(&dh_yx, &dh_bx, &point.auth_key, &point.enc_key, &ephemeral.public, &server, &server_auth)
    else {
        trace::say(b"onion rendezvous handshake did not verify");
        job.outcome = Some(Outcome::Failed(1));
        return;
    };
    let identity = job.lookup.identity;
    let (stream, port) = (job.stream, job.port);
    let circuit = &mut state.circuits[index];
    circuit.hops.push(Hop::onion(&keys));
    for byte in keys.iter_mut() {
        /* SAFETY: eK@nonos.systems. The hop keys are in the hop now. */
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    circuit.service = Some(identity);
    let circuit_id = circuit.id;
    if send_relay(state, index, RELAY_BEGIN, stream, &begin_onion(port)).is_err() {
        state.onion[at].outcome = Some(Outcome::Failed(1));
        return;
    }
    if let Some(s) = state.streams.iter_mut().find(|s| s.id == stream) {
        s.circuit = circuit_id;
    }
    state.onion[at].outcome = Some(Outcome::Connected);
    trace::say_num(b"onion service reached, stream begin sent", stream as u64);
}
