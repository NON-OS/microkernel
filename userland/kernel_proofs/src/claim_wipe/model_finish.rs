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

//! step.rs's gathering of the due records, and finish.rs `put` once its
//! frames are settled (model_end). The busy mark is the stepping token
//! (undo check 10 takes it away); a record is found again by its epoch and
//! given what the act left, or what its end leaves (undo check 6 trusts a
//! display's clean reset).

use super::due_rule::Due;
use super::end_rule::{ended, End};
use super::model_stepper::{due_at, Stepper};
use super::world::World;
use super::world_cases::{BUSY, CLEAN, LOST, UNRESET};
use super::world_knob::Knob;
use super::world_types::Wipe;

pub(super) fn collect(w: &mut World, s: &mut Stepper, me: usize) {
    w.wipes_by = Some(me);
    let (knob, now) = (w.knob, s.now);
    let due = |r: &&mut Wipe| due_at(knob, r.phase, now) != Due::Wait;
    let Some(r) = w.rec.as_mut().filter(due) else { return };
    if r.busy && knob != Knob::NoToken {
        w.seen[BUSY] += 1;
        return;
    }
    r.busy = true;
    s.job = Some(*r);
}

pub(super) fn put(w: &mut World, s: &mut Stepper, me: usize, now: u64) {
    let Some(job) = s.job else { return };
    w.wipes_by = Some(me);
    if w.rec.is_none_or(|r| r.epoch != job.epoch) {
        s.end = None;
        return;
    }
    let display = w.dev.caps.display && w.knob != Knob::DisplayTrusted;
    w.rec = match s.end.map(|e| ended(e, display, now)) {
        None => Some(Wipe { busy: false, ..job }),
        Some(None) => None,
        Some(Some(phase)) => Some(Wipe { phase, busy: false, ..job }),
    };
    let Some(end) = s.end else { return };
    w.pending = w.pending.saturating_sub(1);
    w.truth.last_end = Some(end);
    if end == End::Lost {
        w.truth.lost_at = Some(w.now);
    }
    let seen = match end {
        End::Clean => CLEAN,
        End::Unreset => UNRESET,
        End::Lost => LOST,
    };
    w.seen[seen] += 1;
}
