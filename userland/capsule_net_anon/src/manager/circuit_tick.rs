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

//! Starting a circuit when none is ready, and giving up on a slow one.

use nonos_libc::mk_uptime_ms;

use crate::circuit::{ask_create, client_circuit_id, BuildError, Circuit, CircuitStage, Purpose};
use crate::circuit::{Pending, HOP_MS};
use crate::path::draw_path_through;
use crate::protocol::CIRCUIT_MAX;
use crate::trace;

use super::circuit_answer::failed;
use super::state::Manager;

/// Start one circuit if there is room and a link, or time out the one being
/// built. Replies are finished by the pump as they arrive, never waited for.
pub fn tick(state: &mut Manager, now: u64) {
    /* Onion circuits are built, timed and torn down by their lookup, so only
     * general ones are counted or timed here. */
    let building = state
        .circuits
        .iter()
        .position(|c| c.purpose == Purpose::General && c.stage == CircuitStage::Handshaking);
    if let Some(index) = building {
        let late =
            state.circuits[index].pending.as_ref().is_none_or(|p| mk_uptime_ms() >= p.deadline);
        if late {
            failed(state, index, BuildError::Timeout);
        }
        return;
    }
    let general = state.circuits.iter().filter(|c| c.purpose == Purpose::General).count();
    if !state.usable_at(now) || general >= CIRCUIT_MAX {
        return;
    }
    let (Some(link), Some(guard)) = (state.link.as_mut(), state.guard.as_ref()) else { return };
    let Some(path) = draw_path_through(&guard.relay, &state.relays, &state.weights) else {
        trace::say(b"no exit and middle to go with the guard");
        return;
    };
    let id = client_circuit_id(state.next_circuit);
    state.next_circuit = state.next_circuit.wrapping_add(1);
    let asked = ask_create(link, id, &path[0]);
    let mut circuit = Circuit::new(id, path);
    match asked {
        Ok(handshake) => {
            let deadline = mk_uptime_ms().saturating_add(HOP_MS);
            circuit.pending = Some(Pending { handshake, deadline });
            state.circuits.push(circuit);
        }
        Err(why) => {
            state.circuits.push(circuit);
            let index = state.circuits.len() - 1;
            failed(state, index, why);
        }
    }
}
