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


//! The rendezvous point and the introductions (hs_client.c and
//! hs_circuit.c on the client side).

use crate::crypto::Ephemeral;
use crate::onion::cells::{introduce1, link_specifiers, RELAY_ESTABLISH_RENDEZVOUS, RELAY_INTRODUCE1};
use crate::onion::desc::IntroPoint;
use crate::onion::hs_ntor::intro_keys;
use crate::onion::pow::extension;
use crate::path::{draw_middle, Flags, Relay};
use crate::trace;

use super::super::out::send_relay;
use super::super::state::Manager;
use super::circuits::{drop_circuit, open, start};
use super::job::{OnionJob, PowState, Stage};
use super::pow;

/// One rendezvous circuit, or one introduction and its answer, gets this
/// long; the service then gets as long again to reach the rendezvous point.
const STEP_SECONDS: u64 = 15;
/// CONNECTREFUSED: the service could not be reached through any of its
/// introduction points.
const REASON_CONNECTREFUSED: u8 = 3;

pub(super) fn step(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    match job.stage {
        Stage::Rendezvous => rendezvous(state, job, now),
        Stage::Introduce => introduce(state, job, now),
        Stage::Waiting => waiting(state, job, now),
        Stage::Directory => Ok(()),
    }
}

/*
 * The rendezvous point is any relay the client would use as a middle, drawn
 * apart from the guard. It learns nothing about which service is being
 * reached: it only matches a cookie.
 */
fn rendezvous(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    let Some(id) = job.rend else {
        let guard = state.guard.as_ref().ok_or(REASON_CONNECTREFUSED)?.relay.clone();
        let point = draw_middle(&state.relays, &state.weights, &[&guard]).ok_or(REASON_CONNECTREFUSED)?;
        job.rend = Some(start(state, &point).ok_or(REASON_CONNECTREFUSED)?);
        job.rendezvous_point = Some(point);
        job.sent = false;
        job.step_deadline = now.saturating_add(STEP_SECONDS);
        return Ok(());
    };
    if now >= job.step_deadline {
        trace::say_num(b"onion rendezvous point not established in time", id as u64);
        return restart_rendezvous(state, job);
    }
    match open(state, id) {
        None => restart_rendezvous(state, job),
        Some(false) => Ok(()),
        Some(true) if job.sent => Ok(()),
        Some(true) => {
            let index = state.circuits.iter().position(|c| c.id == id).ok_or(REASON_CONNECTREFUSED)?;
            let cookie = job.cookie;
            if send_relay(state, index, RELAY_ESTABLISH_RENDEZVOUS, 0, &cookie).is_err() {
                return restart_rendezvous(state, job);
            }
            job.sent = true;
            Ok(())
        }
    }
}

/// Drop the rendezvous circuit and build another; the overall deadline
/// bounds how often.
fn restart_rendezvous(state: &mut Manager, job: &mut OnionJob) -> Result<(), u8> {
    if let Some(id) = job.rend.take() {
        drop_circuit(state, id);
    }
    if let Some(id) = job.circuit.take() {
        drop_circuit(state, id);
    }
    /* A refetch in progress keeps going; only the rendezvous point is
     * built again. */
    if job.stage != Stage::Directory {
        job.stage = Stage::Rendezvous;
    }
    job.rend_ready = false;
    job.sent = false;
    job.ephemeral = None;
    job.introduced = None;
    Ok(())
}

fn introduce(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    if job.rend.is_none_or(|id| open(state, id) != Some(true)) {
        return restart_rendezvous(state, job);
    }
    let Some(id) = job.circuit else {
        return next_intro(state, job, now);
    };
    /* The puzzle is solved while the introduction circuit builds, and the
     * step's clock waits for it: the circuit is not late while we are the
     * ones still working. */
    let ready = pow::ready(job, now, STEP_SECONDS);
    if !ready {
        job.step_deadline = job.step_deadline.max(now.saturating_add(STEP_SECONDS));
    }
    if now >= job.step_deadline {
        trace::say_num(b"onion introduction not answered in time", id as u64);
        return give_up_intro(state, job, now);
    }
    match open(state, id) {
        None => give_up_intro(state, job, now),
        Some(false) => Ok(()),
        Some(true) if job.sent || !ready => Ok(()),
        Some(true) => send_introduce(state, job, id, now),
    }
}

fn next_intro(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    while let Some(point) = job.intro_points.pop() {
        if let Some(id) = start(state, &as_relay(&point)) {
            job.circuit = Some(id);
            job.introduced = Some(point);
            job.sent = false;
            job.step_deadline = now.saturating_add(STEP_SECONDS);
            return Ok(());
        }
    }
    if job.from_cache {
        /* Cached points that all fail: the service has published new ones.
         * Forget the entry and ask the HSDirs before giving up. */
        trace::say(b"onion cached introduction points all failed, refetching the descriptor");
        state.desc_cache.forget(&job.lookup.blinded, job.lookup.period);
        job.from_cache = false;
        job.stage = Stage::Directory;
        return Ok(());
    }
    trace::say(b"onion service unreachable through every introduction point");
    Err(REASON_CONNECTREFUSED)
}

fn give_up_intro(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    if let Some(id) = job.circuit.take() {
        drop_circuit(state, id);
    }
    /* A point that did not get us through raises the next puzzle's effort,
     * whether it failed before or after the introduction went out. A
     * solution not yet sent is kept: its nonce has not been spent. */
    if job.introduced.is_some() {
        job.unreachable = job.unreachable.saturating_add(1);
    }
    job.ephemeral = None;
    job.introduced = None;
    next_intro(state, job, now)
}

/// INTRODUCE1 through the open introduction circuit. The ephemeral key is
/// kept: RENDEZVOUS2 is finished with it.
fn send_introduce(state: &mut Manager, job: &mut OnionJob, circuit: u32, now: u64) -> Result<(), u8> {
    let (Some(point), Some(rp)) = (job.introduced.clone(), job.rendezvous_point.clone()) else {
        return give_up_intro(state, job, now);
    };
    let Ok(ephemeral) = Ephemeral::generate() else {
        return give_up_intro(state, job, now);
    };
    let Ok(dh_bx) = ephemeral.shared(&point.enc_key) else {
        return give_up_intro(state, job, now);
    };
    let keys = intro_keys(&dh_bx, &point.auth_key, &ephemeral.public, &point.enc_key, &job.lookup.subcredential);
    let specs = link_specifiers(rp.address, rp.or_port, &rp.rsa_identity, &rp.ed25519_identity);
    let field = match &job.pow_state {
        PowState::Solved(solution) => Some(extension(solution)),
        _ => None,
    };
    let Some(cell) = introduce1(&point.auth_key, &ephemeral.public, &keys, &job.cookie, (&rp.ntor_onion_key, &specs), field.as_ref()) else {
        return give_up_intro(state, job, now);
    };
    let Some(index) = state.circuits.iter().position(|c| c.id == circuit) else {
        return give_up_intro(state, job, now);
    };
    if send_relay(state, index, RELAY_INTRODUCE1, 0, &cell).is_err() {
        return give_up_intro(state, job, now);
    }
    job.ephemeral = Some(ephemeral);
    job.sent = true;
    /* The nonce is spent; the next introduction solves afresh. */
    job.pow_state = PowState::Idle;
    trace::say_num(b"onion introduce sent", circuit as u64);
    Ok(())
}

/// Introduced: the service has STEP_SECONDS to reach the rendezvous point.
/// One that does not is tried through the next introduction point, with the
/// same cookie, which the rendezvous point still holds.
fn waiting(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    if job.rend.is_none_or(|id| open(state, id) != Some(true)) {
        return restart_rendezvous(state, job);
    }
    if job.step_deadline == 0 {
        job.step_deadline = now.saturating_add(STEP_SECONDS);
    }
    if now >= job.step_deadline {
        trace::say(b"onion service did not reach the rendezvous point in time");
        job.stage = Stage::Introduce;
        return give_up_intro(state, job, now);
    }
    Ok(())
}

/// An introduction point as a relay to extend to. It need not be in the
/// consensus: the descriptor names it, and the EXTEND2 to it carries all
/// three identities, so a relay at that address with other keys refuses.
fn as_relay(point: &IntroPoint) -> Relay {
    Relay {
        address: point.address,
        or_port: point.port,
        rsa_identity: point.rsa_identity,
        ed25519_identity: point.ed25519_identity,
        ntor_onion_key: point.onion_key,
        flags: Flags::default(),
        weight: 0,
        exits_web: false,
    }
}
