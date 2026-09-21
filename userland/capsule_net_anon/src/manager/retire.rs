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

//! Dropping circuits that should take no more streams.

extern crate alloc;

use alloc::vec::Vec;

use crate::circuit::{destroy, CircuitStage};
use crate::protocol::{CIRCUIT_DIRTY_SECONDS, CIRCUIT_FAILURES_MAX};
use crate::trace;

use super::state::Manager;

/*
 * A spent circuit has to actually go, not merely stop being chosen. While it sits
 * in the table it counts against CIRCUIT_MAX, so a client that had built its
 * three and retired two would never draw a replacement and would be left with the
 * one path it had.
 *
 * A circuit with live streams on it is left alone whatever its age. Those streams
 * are a page being read, and tearing the circuit down under them would fail a
 * request that was working.
 */

/// Drop every circuit too old or too broken for new streams and carrying none.
pub fn tick(state: &mut Manager, now: u64) {
    let mut going: Vec<u32> = Vec::new();
    for circuit in state.circuits.iter() {
        if circuit.stage != CircuitStage::Open || !spent(circuit.failures, circuit.opened_at, now) {
            continue;
        }
        if state.streams.iter().any(|s| s.circuit == circuit.id && s.stage.is_live()) {
            continue;
        }
        going.push(circuit.id);
    }
    if going.is_empty() {
        return;
    }
    /*
     * DESTROY first, while the circuit is still in the table and the link is still
     * the one that carries it. A failed send is not worth acting on: the circuit is
     * being dropped either way, and a link too broken to take a DESTROY will take
     * every circuit on it with it in a moment anyway.
     */
    if let Some(link) = state.link.as_mut() {
        for id in going.iter() {
            let _ = link.send(&destroy(*id).encode());
        }
    }
    state.circuits.retain(|c| !going.contains(&c.id));
    trace::say_num(b"circuits retired", going.len() as u64);
}

fn spent(failures: u8, opened_at: u64, now: u64) -> bool {
    failures >= CIRCUIT_FAILURES_MAX || now >= opened_at.saturating_add(CIRCUIT_DIRTY_SECONDS)
}
