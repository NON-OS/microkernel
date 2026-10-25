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

//! The invariants of the enter model (L78), by name: those every step is
//! held to here, those its claims' steps check where they act, and those
//! only a world where nothing can move is held to.

use super::enter_parts::{Entry, END, IN};
use super::enter_stage::Stage;
use super::enter_world::World;

pub(super) const I1: &str = "I1: admitted over another service's state";
pub(super) const I1_LOST: &str = "I1: admitted within a Lost hold";
pub(super) const I5: &str = "I5: a Making or Ready entry while its wipe is pending";
pub(super) const I7: &str = "I7: a withdrawn claim changed the wipe record";
pub(super) const I9: &str = "I9: told Resetting with no wipe pending and no entry going";
pub(super) const I10: &str = "I10: told AlreadyClaimed with no claim standing";
pub(super) const COUNT: &str = "PENDING miscounts the pending records";
pub(super) const PARKED: &str = "a path that can never move again";
pub(super) const STRANDED: &str = "an entry stranded";
pub(super) const LEFT: &str = "a wipe left pending";

impl World {
    pub(super) fn fail(&mut self, why: &'static str) {
        self.bad = self.bad.or(Some(why));
    }
    /// After every step: I5, and PENDING counting the one pending record.
    pub(super) fn check(&mut self) {
        let live = |s: Stage| matches!(s, Stage::Making | Stage::Ready);
        if self.pending() && self.entries.iter().flatten().any(|e| live(e.stage)) {
            self.fail(I5);
        }
        if self.count != u8::from(self.pending()) {
            self.fail(COUNT);
        }
    }
    /// Where nothing can move: every path done, no entry left but a Ready
    /// one its claimant was admitted to, and no wipe left pending.
    pub(super) fn end_broken(&self) -> Vec<&'static str> {
        let held = |c: usize, e: &Entry| c > 0 && e.stage == Stage::Ready && self.pc[c + 2] == IN;
        let stranded =
            self.entries.iter().enumerate().any(|(c, e)| e.is_some_and(|e| !held(c, &e)));
        let parked = [0, 1, 3, 4].iter().any(|&p| self.pc[p] < END);
        let left = self.pending() || self.count > 0;
        [(parked, PARKED), (stranded, STRANDED), (left, LEFT)]
            .iter()
            .filter(|b| b.0)
            .map(|b| b.1)
            .collect()
    }
}
