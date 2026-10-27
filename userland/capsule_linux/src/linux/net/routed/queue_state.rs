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

//! What a stream's queue can say of itself: whether a frame is out, what
//! is still to go, and how much more it takes.

use super::queue::{Queue, TX_MAX};

impl Queue {
    /// A frame is out unanswered: nothing may be read until it is.
    pub fn out(&self) -> bool {
        self.offered != 0
    }

    /// Bytes written that have not gone yet, and still can.
    pub fn queued(&self) -> bool {
        !self.broken && !self.bytes.is_empty()
    }

    pub fn broken(&self) -> bool {
        self.broken
    }

    pub fn room(&self) -> usize {
        if self.broken {
            0
        } else {
            TX_MAX - self.bytes.len()
        }
    }
}
