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

//! The enter model's clock (L78): it moves only to the next instant some
//! rule's answer, or the device's, can change. Between two such instants
//! every answer is the same, so no order is lost by skipping the time
//! between; the seeded model tries the times in between.

use super::due_rule::deadline;
use super::enter_world::World;
use super::phase_rule::{Phase, FLR_WAIT, LOST_HOLD, PM_WAIT};

/// The instants a record's phase can change its rules' answers at.
fn marks(p: Phase) -> [Option<u64>; 2] {
    match p {
        Phase::Flr { at } => [Some(at + FLR_WAIT), deadline(p)],
        Phase::D3 { at } | Phase::D0 { at } => [Some(at + PM_WAIT), deadline(p)],
        Phase::Lost { at } => [Some(at + LOST_HOLD), None],
        _ => [None, None],
    }
}

impl World {
    /// On to the next such instant; none ahead, the clock stands.
    pub(super) fn advance(&mut self) -> bool {
        let back = self.written.map(|t| t.saturating_add(self.fate.back));
        let phases = [self.rec.map(|r| r.phase), self.job.map(|j| j.rec.phase)];
        let ahead = phases.into_iter().flatten().flat_map(marks).chain([back]).flatten();
        let Some(t) = ahead.filter(|&t| t > self.now && t < u64::MAX).min() else {
            return false;
        };
        self.now = t;
        true
    }
}
