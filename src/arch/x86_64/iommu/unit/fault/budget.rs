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

//! How many fault lines one poll may print. A device misprogrammed into a
//! loop faults on every transfer, thousands a second, and printing each one
//! would drown the log and hold the serial lock from the timer interrupt. The
//! first few name the device; the rest are counted.

/// Lines per poll, and a poll runs about once a second.
pub const LINES_PER_POLL: u32 = 8;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    pub shown: u32,
    pub hidden: u32,
}

impl Budget {
    /// Whether the next record gets its own line; counts it either way.
    pub fn admit(&mut self) -> bool {
        if self.shown < LINES_PER_POLL {
            self.shown += 1;
            true
        } else {
            self.hidden = self.hidden.saturating_add(1);
            false
        }
    }
}
