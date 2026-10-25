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

//! The randomness the layout is drawn from: a fresh value for every offset,
//! from the CPU's instruction or else the seeded RNG. When neither answers
//! the layout is refused, never guessed: a constant seed or a counter
//! would place every program at the same addresses on every such boot.

use super::state::AslrManager;
use crate::arch::cpu_random;

pub(super) fn draw() -> Option<u64> {
    cpu_random::random_u64().or_else(|| crate::crypto::rng::random_u64_secure().ok())
}

impl AslrManager {
    /// An offset below `max_offset`, or `None` when no randomness answers.
    pub fn random_offset(&mut self, max_offset: u64) -> Option<u64> {
        if max_offset == 0 {
            return Some(0);
        }
        self.entropy_pool ^= draw()?;
        Some((self.entropy_pool >> 16) % max_offset)
    }

    /// Mix a fresh value into the pool; false when none could be drawn.
    pub fn reseed(&mut self) -> bool {
        draw().map(|v| self.entropy_pool ^= v).is_some()
    }
    pub fn entropy(&self) -> u64 {
        self.entropy_pool
    }
}
