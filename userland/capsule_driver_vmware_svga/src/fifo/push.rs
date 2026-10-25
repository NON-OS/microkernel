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

//! The driver's side of the ring: how much room is left, and writing a
//! command into it.

use core::sync::atomic::{fence, Ordering};

use super::ring::Fifo;
use super::space::free_bytes;
use crate::constants::fifo::{FIFO_NEXT_CMD, FIFO_STOP};

impl Fifo {
    /// Free bytes now, or None when the device's indices are out of the ring.
    pub fn free(&self) -> Option<u32> {
        free_bytes(self.get(FIFO_NEXT_CMD), self.get(FIFO_STOP), self.min, self.max)
    }

    /// Write `words` one at a time, moving NEXT_CMD after each, so a command
    /// that wraps past the end is never seen half written: the device reads
    /// only up to NEXT_CMD. This is VMware's own path for a FIFO without
    /// the reserve capability.
    pub fn push(&self, words: &[u32]) {
        for &w in words {
            let next = self.get(FIFO_NEXT_CMD);
            self.set(next as usize / 4, w);
            let after = if next + 4 >= self.max { self.min } else { next + 4 };
            fence(Ordering::Release);
            self.set(FIFO_NEXT_CMD, after);
        }
    }
}
