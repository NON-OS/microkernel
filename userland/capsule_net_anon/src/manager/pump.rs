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

//! Reading cells off the link and routing them to their circuit.

use crate::cell::{Frame, CELL_DESTROY, CELL_RELAY, CELL_RELAY_EARLY};
use crate::circuit::{open, CircuitStage};
use crate::trace;

use super::inbound::deliver;
use super::link_lost::lost;
use super::state::Manager;

const IDLE_MS: i64 = 50;

/// Read whatever has arrived and route it. One cell per call keeps the serve loop
/// responsive under a circuit that is delivering hard.
pub fn tick(state: &mut Manager) {
    let Some(link) = state.link.as_mut() else { return };
    let frame = match link.recv(IDLE_MS) {
        Ok(Some(frame)) => frame,
        Ok(None) => return,
        Err(_) => {
            lost(state);
            return;
        }
    };
    let Frame::Fixed(mut cell) = frame else {
        /*
         * A variable length cell after the handshake is padding or a version
         * negotiation cell, with nothing in it that belongs to a circuit.
         */
        return;
    };
    let Some(index) = state.circuits.iter().position(|c| c.id == cell.circuit) else {
        return;
    };
    if cell.command == CELL_DESTROY {
        trace::say_num(b"circuit destroyed by the far end", cell.circuit as u64);
        state.circuits[index].stage = CircuitStage::Dead;
        return;
    }
    if cell.command != CELL_RELAY && cell.command != CELL_RELAY_EARLY {
        return;
    }
    let Some(opened) = open(&mut state.circuits[index].hops, &mut cell.payload) else {
        /*
         * No hop recognised it. That is not a cell to guess about: a relay cell
         * whose digest matches no hop either came from somewhere else or the
         * chain has desynchronised, and either way the circuit is finished.
         */
        trace::say_num(b"unrecognised cell, circuit dead", cell.circuit as u64);
        state.circuits[index].stage = CircuitStage::Dead;
        return;
    };
    deliver(state, index, opened.hop, &cell.payload);
}
