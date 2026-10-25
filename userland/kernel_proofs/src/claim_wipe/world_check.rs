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

//! The invariants of plan section 6, and L80's. I1 to I4 and I6 to I10 are
//! judged where they can break (an admit, a free, a config access, a reset
//! write, a withdraw, an attempt's end, libc giving up, the record's answer
//! to a claim); I5 on every move and once more when the seed has run out.
//! Each sets its own bit, so an undo check names its own.

use super::due_rule::pending;
use super::model_stage::types::Stage;
use super::phase_rule::Phase;
use super::world::World;
use super::world_cases::{BOUND, GAP, QUIET_FLR, QUIET_PM, RETRY};

impl World {
    pub(super) fn bad(&mut self, inv: u8, why: &'static str) {
        if self.broke == 0 {
            self.why = why;
        }
        self.broke |= 1 << inv;
    }

    /// I5 on every move: no entry made over a wipe under way, no wipe left
    /// pending past its whole window, nor a gap past its own reset's, no
    /// Held record its claim left behind.
    pub(super) fn check_step(&mut self) {
        let pending = self.rec.is_some_and(|r| pending(r.phase));
        if pending && self.entry.is_some_and(|e| e.stage == Stage::Making) {
            self.bad(5, "a claim's entry made over a wipe under way");
        }
        let window = |p: Phase| match p {
            Phase::Flr { at } => Some(at + QUIET_FLR + RETRY),
            Phase::D0 { at } => Some(at + QUIET_PM + RETRY),
            _ => None,
        };
        let late = self.rec.and_then(|r| window(r.phase)).is_some_and(|d| self.now > d + GAP);
        if late || (pending && self.now.saturating_sub(self.truth.since) > BOUND) {
            self.bad(5, "a wipe left pending past its window");
        }
        let held = self.rec.filter(|r| r.phase == Phase::Held).map(|r| r.epoch);
        if held.is_some_and(|h| self.entry.is_none_or(|e| e.epoch != h)) {
            self.bad(5, "a Held record its claim left behind");
        }
    }

    /// Once nothing is left to move: no claim, wipe or run stranded.
    pub(super) fn check_end(&mut self) {
        let left = self.rec.is_some_and(|r| pending(r.phase) || r.phase == Phase::Held);
        if left || self.entry.is_some() || self.pending != 0 {
            self.bad(5, "a claim or a wipe stranded");
        }
        let kept = |id: usize| self.kept.iter().any(|k| k.0 == id);
        let gone = |id: usize| self.runs[id].frees > 0 || self.runs[id].registry || kept(id);
        if !(0..self.runs.len()).all(gone) {
            self.bad(2, "a run neither freed nor kept");
        }
    }
}
