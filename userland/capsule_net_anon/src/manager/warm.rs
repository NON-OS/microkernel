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

//! Reading the table for the warm circuit's rule (warm_rule.rs), and
//! marking the circuit it draws.

use crate::circuit::{Circuit, CircuitStage, Purpose};
use crate::trace;

use super::state::Manager;
use super::warm_rule::{draw_warm, spent};

/// Whether the general circuit about to be drawn is the warm one. `general`
/// counts every general circuit in the table, warm or not, building or open.
pub(super) fn wanted(state: &Manager, now: u64, general: usize) -> bool {
    let up = state.usable_at(now) && state.link.is_some();
    let held = state.circuits.iter().any(|c| c.warm && takes(c, now, true));
    let others = state.circuits.iter().filter(|c| !c.warm && takes(c, now, false)).count();
    draw_warm(up, general, held, others)
}

/// Whether `c` is a general circuit that takes streams, or, when `building`
/// counts too, one still being built: a warm one being built is held, so a
/// second is not drawn behind it.
fn takes(c: &Circuit, now: u64, building: bool) -> bool {
    let open = c.stage == CircuitStage::Open && !spent(c.warm, c.failures, c.opened_at, now);
    c.purpose == Purpose::General && (open || (building && c.stage == CircuitStage::Handshaking))
}

/// Mark `circuit` as the warm one, and say so on a build that has serial.
pub(super) fn mark(circuit: &mut Circuit, warm: bool) {
    circuit.warm = warm;
    if warm {
        trace::say_num(b"warm circuit for installs, building", circuit.id as u64);
    }
}
