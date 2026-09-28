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

//! Finishing a hop when its reply arrives, and asking for the next one.

use nonos_libc::mk_uptime_ms;

use crate::cell::Cell;
use crate::circuit::{answer, ask_extend, BuildError, CircuitStage, Pending, HOP_MS};
use crate::protocol::HOPS;
use crate::trace;

use super::link_lost::lost;
use super::state::Manager;

/// `cell` arrived for the circuit at `index`, which is being built.
pub fn arrived(state: &mut Manager, index: usize, mut cell: Cell, now: u64) {
    let Manager { link, circuits, .. } = state;
    let circuit = &mut circuits[index];
    let Some(pending) = circuit.pending.take() else {
        return failed(state, index, BuildError::Protocol);
    };
    match answer(&mut cell, &mut circuit.hops, |reply| pending.handshake.finish(reply).ok()) {
        Ok(hop) => circuit.hops.push(hop),
        Err(why) => return failed(state, index, why),
    }
    trace::say_num(b"circuit hop up", circuit.hops.len() as u64);
    if circuit.hops.len() == HOPS {
        circuit.stage = CircuitStage::Open;
        circuit.opened_at = now;
        trace::say_num(b"circuit open", circuit.id as u64);
        return;
    }
    let Some(link) = link.as_mut() else { return failed(state, index, BuildError::Link) };
    let next = circuit.path[circuit.hops.len()].clone();
    match ask_extend(link, circuit.id, &mut circuit.hops, &next) {
        Ok(handshake) => {
            let deadline = mk_uptime_ms().saturating_add(HOP_MS);
            circuit.pending = Some(Pending { handshake, deadline });
        }
        Err(why) => failed(state, index, why),
    }
}

/*
 * A failed build leaves the far end holding whatever hops did come up, and
 * this side cannot tell which. The link is dropped rather than reused: a half
 * built circuit sharing it would have its cells read against the wrong
 * digest for the life of the connection.
 */
pub fn failed(state: &mut Manager, index: usize, why: BuildError) {
    trace::say_num(why.said(), state.circuits[index].id as u64);
    lost(state);
}
