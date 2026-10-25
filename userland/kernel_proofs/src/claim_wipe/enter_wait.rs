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

//! The undone designs' wait inside Making in the enter model (L78): the
//! claimant, its entry pushed over a pending wipe, yields until the wipe
//! ends, and a kill landing in that yield parks the thread for good (L77).
//! Under ResetInMaking the waiting claimant is the only one to step it.

use super::admit_rule::Admit;
use super::enter_events::BY_WAIT;
use super::enter_parts::Knob;
use super::enter_wipe::pass;
use super::enter_world::World;

pub(super) fn wait(w: &mut World, c: u8, k: Knob) -> bool {
    let p = usize::from(c) + 2;
    if w.killed && w.prey == c {
        return false;
    }
    if !w.pending() {
        w.pc[p] = 9;
        return true;
    }
    let stepped = k == Knob::ResetInMaking && pass(w, 0, k, BY_WAIT);
    if stepped {
        w.pc[p] = 11;
    }
    stepped
}

/// After the wait: the record asked again, in one claims hold; a Lost one
/// asked once more costs a try, and with none left the claim withdraws.
pub(super) fn reask(w: &mut World, c: u8) {
    let (i, p) = (usize::from(c), usize::from(c) + 2);
    w.pc[p] = match w.admits(c) {
        Admit::Go => 5,
        Admit::Refused => 7,
        Admit::Rearm if w.tries[i] == 0 => 7,
        Admit::Rearm => {
            w.tries[i] -= 1;
            w.rearm();
            8
        }
        Admit::Resetting => 8,
    };
}
