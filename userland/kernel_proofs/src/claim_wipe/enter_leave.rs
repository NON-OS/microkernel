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

//! A's own paths in the enter model (L78), its give_back and its exit: the
//! last out of the claim removes its entry and owes the wipe, then steps it.

use super::enter_events::BY_A;
use super::enter_parts::{Entry, Knob, END};
use super::enter_settle::{owe_late, settle};
use super::enter_stage::{exited, retired, Stage};
use super::enter_wipe::pass;
use super::enter_world::World;

fn or_end(go: bool, pc: u8) -> u8 {
    [END, pc][usize::from(go)]
}

/// give_back.rs: retire, quiesce_held, the drains, then release::release.
pub(super) fn give_back(w: &mut World, pc: u8, k: Knob) -> bool {
    let a = w.entries[0];
    w.pc[0] = match pc {
        // leave::retire, one claims hold, the release counted; no grant path
        // is inside the claim here, so its wait for them ends at once.
        0 => match a.map(|e| (e, retired(e.stage))) {
            Some((e, Ok(stage))) => {
                w.entries[0] = Some(Entry { stage, leavers: e.leavers + 1, ..e });
                1
            }
            _ => END,
        },
        // quiesce_held (a claims hold) and the drains (the grant tables'):
        // neither changes anything this model holds.
        1 | 2 => pc + 1,
        // release: its own check that the claim is still being released.
        3 => or_end(a.is_some_and(|e| matches!(e.stage, Stage::Leaving | Stage::Exiting)), 4),
        4 => or_end(settle(w, 0, k), 5),
        5 if w.late == Some(0) => owe_late(w, 5),
        // confine::detach, then step_all(true).
        _ => or_end(pass(w, pc - 5, k, BY_A), pc + 1),
    };
    true
}

/// exit/devices.rs: exit_begin, the drains, then exit_end.
pub(super) fn exit(w: &mut World, pc: u8, k: Knob) -> bool {
    w.pc[1] = match pc {
        // exit_begin: one claims hold; a claim it counts is its to settle.
        0 => match w.entries[0].map(|e| (e, exited(e.stage))) {
            Some((e, (stage, counted))) => {
                w.entries[0] = Some(Entry { stage, leavers: e.leavers + u8::from(counted), ..e });
                [3, 1][usize::from(counted)]
            }
            None => 3,
        },
        1 => 2,
        2 => (settle(w, 1, k), 3).1,
        3 if w.late == Some(1) => owe_late(w, 3),
        // exit_end: detach_all, then step_all(true).
        _ => or_end(pass(w, pc - 3, k, BY_A), pc + 1),
    };
    true
}
