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

//! One atomic step of a path in the enter model (L78): one hold of a lock,
//! or one act with none, as the kernel takes it; false when the path cannot
//! move. The tick, the clock, a fault and a kill here; A's give_back and
//! exit in enter_leave, the claimants in enter_claim.

use super::enter_events::BY_TICK;
use super::enter_parts::{Knob, END};
use super::enter_wipe::pass;
use super::enter_world::World;

pub(super) fn step(w: &mut World, p: usize, k: Knob) -> bool {
    let pc = w.pc[p];
    let moved = match p {
        0 => super::enter_leave::give_back(w, pc, k),
        1 => super::enter_leave::exit(w, pc, k),
        2 => tick(w, pc, k),
        3 | 4 => super::enter_claim::claimant(w, p as u8 - 2, k),
        5 => w.advance(),
        // A fault armed for the next claim's middle steps; a kill landed.
        _ => {
            (w.fault, w.pc[p]) = (w.fault.max(u8::from(p == 6)), END);
            w.killed |= p == 7;
            true
        }
    };
    if moved {
        w.check();
    }
    moved
}

// The x86 tick's step_all(false): a pass when anything is due. Its try of
// the table's lock failing is the same as the tick not having come yet.
fn tick(w: &mut World, pc: u8, k: Knob) -> bool {
    let on = pass(w, pc, k, BY_TICK);
    if pc == 0 && !on {
        return false;
    }
    w.pc[2] = if on { pc + 1 } else { 0 };
    true
}
