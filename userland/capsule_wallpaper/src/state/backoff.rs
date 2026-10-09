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

//! When to ask the compositor again, counted in turns of the service loop
//! (about a frame each). A call the compositor did not answer is asked again,
//! never given up on, but at a growing interval: a compositor busy composing
//! is not helped by being asked every frame. Pure; held in
//! wallpaper_catalog_proofs.

/// Turns to wait after the first failure.
pub const FIRST_WAIT: u32 = 8;
/// The longest wait, about four seconds at 60 Hz.
pub const MAX_WAIT: u32 = 256;

pub struct Backoff {
    /// Turns left before the next try.
    left: u32,
    /// The wait after the next failure.
    next: u32,
}

impl Default for Backoff {
    fn default() -> Self {
        Self::new()
    }
}

impl Backoff {
    pub const fn new() -> Self {
        Self { left: 0, next: FIRST_WAIT }
    }

    /// Each turn: whether to try now.
    pub fn due(&mut self) -> bool {
        if self.left == 0 {
            return true;
        }
        self.left -= 1;
        false
    }

    /// The try failed: wait, longer each time up to `MAX_WAIT`.
    pub fn failed(&mut self) {
        self.left = self.next;
        self.next = self.next.saturating_mul(2).min(MAX_WAIT);
    }

    /// The try went through: the next failure waits the shortest again.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
