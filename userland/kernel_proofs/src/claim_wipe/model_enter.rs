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

//! enter.rs `enter`'s claims hold: an entry already there answers by its
//! stage (stage_rule `over`); else the record is asked, then the kept bound
//! in that same hold (model_table), and a Go makes the entry in the claims
//! hold; a lost function past its hold is asked again (undo check 13 refuses
//! it instead). pending.rs `withdraw` takes back an entry made, and must
//! leave the record exactly as it was (I7).

use super::admit_rule::Admit;
use super::model_attempt::{attempt_over, refused};
use super::model_stage::stage_rule::over;
use super::model_stage::types::{ClaimError, Stage};
use super::world::World;
use super::world_cases::{CAP, WITHDRAWN};
use super::world_knob::Knob;
use super::world_thread::Thread;
use super::world_types::Entry;

/// enter.rs `KEEP_CAP` (our bound).
pub(super) const KEEP_CAP: u64 = 256 << 20;

pub(super) fn enter(w: &mut World, p: usize, t: &mut Thread) {
    if let Some(e) = w.entry {
        return refused(w, p, t, over(e.stage));
    }
    match w.admits(p) {
        Admit::Go => {
            (t.epoch, w.epochs) = (w.epochs, w.epochs + 1);
            let (pid, epoch, stage) = (p, t.epoch, Stage::Making);
            w.entry = Some(Entry { pid, epoch, stage, leavers: 0 });
            w.procs[p].before = w.rec;
        }
        Admit::Rearm if w.knob != Knob::NoRearm => {
            w.rearm();
            refused(w, p, t, ClaimError::Resetting);
        }
        Admit::Resetting => refused(w, p, t, ClaimError::Resetting),
        Admit::Rearm | Admit::Refused => {
            w.seen[CAP] += w.procs[p].over_cap as u32;
            refused(w, p, t, ClaimError::Refused);
        }
    }
}

pub(super) fn ours(w: &World, p: usize, epoch: u64) -> bool {
    w.entry.is_some_and(|e| e.pid == p && e.epoch == epoch)
}

pub(super) fn withdraw(w: &mut World, p: usize, t: &mut Thread) {
    if ours(w, p, t.epoch) {
        w.entry = None;
    }
    if w.rec != w.procs[p].before {
        w.bad(7, "a withdrawn claim changed the record");
    }
    w.seen[WITHDRAWN] += 1;
    attempt_over(w, p, t);
}
