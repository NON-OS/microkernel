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

//! A claimant told Resetting in the enter model (L78), and its push of
//! Making. libc asks again after a while in user code, never in the kernel,
//! so a kill there only ends the call; the record as the push found it is
//! kept, for I7 to hold a withdraw to.

use super::enter_check::{I10, I9};
use super::enter_events::{AGAIN, REARMED, TAKEN};
use super::enter_parts::{Entry, Knob, OUT};
use super::enter_record::service;
use super::enter_stage::{over, ClaimError, Stage};
use super::enter_world::World;

/// I9: a claimant told to wait has a pending wipe, or an entry going, to
/// wait for; then libc's next try, if it has one and was not killed.
pub(super) fn again(w: &mut World, c: u8) {
    let going = w.entries.iter().flatten().any(|e| over(e.stage) == ClaimError::Resetting);
    if !going && !w.pending() {
        w.fail(I9);
    }
    let i = usize::from(c);
    w.events |= AGAIN;
    if w.killed && w.prey == c || w.tries[i] == 0 {
        w.pc[i + 2] = OUT;
    } else {
        (w.pc[i + 2], w.tries[i]) = (0, w.tries[i] - 1);
    }
}

/// AlreadyClaimed, which libc does not ask again on. I10: said only while a
/// claim stands on the function; one going says Resetting instead.
pub(super) fn taken(w: &mut World, c: u8) {
    let stands = |s: Stage| matches!(s, Stage::Making | Stage::Ready);
    if !w.entries.iter().flatten().any(|e| stands(e.stage)) {
        w.fail(I10);
    }
    (w.events, w.pc[usize::from(c) + 2]) = (w.events | TAKEN, OUT);
}

/// enter's Rearm: the Lost record asked once more, and Resetting said.
pub(super) fn rearm(w: &mut World, c: u8) {
    w.rearm();
    w.events |= REARMED;
    again(w, c);
}

/// enter's push of Making; HeldAtEnter writes the claim's record here too.
pub(super) fn push(w: &mut World, c: u8, k: Knob) {
    let i = usize::from(c);
    let e = Entry { epoch: w.epoch, stage: Stage::Making, leavers: 0 };
    (w.entries[i], w.snap[i]) = (Some(e), w.rec);
    (w.epoch, w.pc[i + 2]) = (w.epoch + 1, 5);
    if k == Knob::HeldAtEnter {
        w.held(e.epoch, service(c));
    }
}
