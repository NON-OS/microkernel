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

use super::seen::Rejoin;
use super::Phase;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Machine {
    pub(super) phase: Phase,
    pub(super) next_ms: i64,
    pub(super) tried: u8,
    /// Saved networks whose join was refused for good this boot.
    pub(super) refused: u8,
    /// Rounds of the saved networks begun after the first.
    pub(super) rounds: u8,
    pub(super) empty_passes: u8,
    pub(super) link: Rejoin,
}

impl Machine {
    pub const fn new() -> Self {
        Self {
            phase: Phase::Waiting,
            next_ms: 0,
            tried: 0,
            refused: 0,
            rounds: 0,
            empty_passes: 0,
            link: Rejoin::NEW,
        }
    }

    /// The saved networks already tried, one bit each.
    pub fn tried(&self) -> u8 {
        self.tried
    }

    pub fn done(&self) -> bool {
        self.phase == Phase::Done
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
