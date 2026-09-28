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

//! Building a circuit when none is ready.

use crate::circuit::{build, client_circuit_id, Circuit, CircuitStage};
use crate::path::draw_path;
use crate::protocol::CIRCUIT_MAX;
use crate::trace;

use super::link_lost::lost;
use super::state::Manager;

/// Build one circuit if there is room and a link to build it over.
///
pub fn tick(state: &mut Manager, now: u64) {
    if !state.usable_at(now) || state.circuits.len() >= CIRCUIT_MAX {
        return;
    }
    if state.circuits.iter().any(|c| c.stage == CircuitStage::Handshaking) {
        return;
    }
    let Some(path) = draw_path(&state.relays, &state.weights) else {
        trace::say(b"no three hop path in the consensus");
        return;
    };
    let Some(link) = state.link.as_mut() else { return };
    let id = client_circuit_id(state.next_circuit);
    state.next_circuit = state.next_circuit.wrapping_add(1);

    match build(link, id, &path) {
        Ok(hops) => {
            let mut circuit = Circuit::new(id, path);
            circuit.hops = hops;
            circuit.stage = CircuitStage::Open;
            circuit.opened_at = now;
            trace::say_num(b"circuit open", id as u64);
            state.circuits.push(circuit);
        }
        Err(_) => {
            /*
             * A failed build leaves the far end holding whatever hops did come
             * up, and this side cannot tell which. The link is dropped rather
             * than reused: a half built circuit sharing it would have its cells
             * read against the wrong digest for the life of the connection.
             */
            trace::say_num(b"circuit build failed", id as u64);
            lost(state);
        }
    }
}
