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

//! The machine, and what it says is due.

use super::rejoin::Rejoin;
use super::{Due, Phase, NOT_YET_MS};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Machine {
    pub(super) phase: Phase,
    pub(super) next_ms: i64,
    pub(super) tried: u8,
    pub(super) empty_passes: u8,
    pub(super) link: Rejoin,
}

impl Machine {
    pub const fn new() -> Self {
        Self { phase: Phase::Waiting, next_ms: 0, tried: 0, empty_passes: 0, link: Rejoin::NEW }
    }

    /// The saved networks already tried, one bit each.
    pub fn tried(&self) -> u8 {
        self.tried
    }

    pub fn done(&self) -> bool {
        self.phase == Phase::Done
    }

    pub fn due(&self, now_ms: i64) -> Due {
        if now_ms < self.next_ms {
            return Due::Nothing;
        }
        match self.phase {
            Phase::Waiting => Due::Attempt,
            Phase::Joining { .. } => Due::Watch,
            Phase::Done => Due::Nothing,
        }
    }

    /// A link is already bound: leave it alone and look again later.
    pub fn bound(&mut self, now_ms: i64) {
        if self.phase == Phase::Waiting {
            self.next_ms = now_ms + NOT_YET_MS;
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) fn bit(index: u8) -> u8 {
    1u8.checked_shl(u32::from(index)).unwrap_or(0)
}
