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

//! A stepper's pass over the wipe in the enter model (L78), as step.rs runs
//! it: one hold of the table that marks the due record busy and copies it,
//! one act on the device with no lock held (enter_act), one hold that puts
//! it back as finish.rs does.

use super::due_rule::{due, Due};
use super::end_rule::{ended, End};
use super::enter_events::{BY_WAIT, CLEAN, LOST, UNRESET};
use super::enter_parts::{Job, Knob, Rec};
use super::enter_world::World;

/// The pass's step `at` (0 take, 1 act, 2 put) by stepper `by`: whether it
/// goes on. Under ResetInMaking only the claimant waiting in Making steps.
pub(super) fn pass(w: &mut World, at: u8, k: Knob, by: u16) -> bool {
    match at {
        0 => take(w, k == Knob::ResetInMaking && by != BY_WAIT),
        1 => super::enter_act::act(w),
        _ => put(w, by),
    }
}

// step_all: PENDING read, then the table's hold; taken as one step, since a
// stale read only returns early or finds the record no longer due.
fn take(w: &mut World, barred: bool) -> bool {
    let (now, count) = (w.now, w.count);
    let Some(r) = w.rec.as_mut() else { return false };
    if barred || count == 0 || r.busy || due(r.phase, now) == Due::Wait {
        return false;
    }
    r.busy = true;
    w.job = Some(Job { rec: *r, end: None, now });
    true
}

// finish::put: an Unreset's marker put back with no lock (check::unmark),
// then the table's hold: the record kept, removed, or left Kept or Lost.
// One step, since no other path reads the bit while the record is busy;
// the kept frames' settle after it touches nothing modeled here.
fn put(w: &mut World, by: u16) -> bool {
    let Some(j) = w.job.take() else { return false };
    w.marker &= j.end != Some(End::Unreset);
    if w.rec.map(|r| r.epoch) == Some(j.rec.epoch) {
        w.rec = match j.end.map(|e| ended(e, w.fate.display, j.now)) {
            None => Some(Rec { busy: false, ..j.rec }),
            Some(None) => None,
            Some(Some(phase)) => Some(Rec { phase, busy: false, ..j.rec }),
        };
        w.count = w.count.saturating_sub(u8::from(j.end.is_some()));
        let seen = [(End::Clean, CLEAN), (End::Unreset, UNRESET), (End::Lost, LOST)];
        w.events |= seen.iter().filter(|e| j.end == Some(e.0)).map(|e| e.1 | by).sum::<u16>();
        w.lost = if j.end == Some(End::Lost) { Some(j.now) } else { w.lost };
    }
    false
}
