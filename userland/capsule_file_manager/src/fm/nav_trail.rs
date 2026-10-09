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

//! What the header's Forward arrow goes back down into. Back goes up one
//! folder, so Forward retraces those steps: each folder Back left is kept,
//! the latest on top, and Forward opens it again. The arrow was drawn, always
//! dim, and a click on it did nothing.

extern crate alloc;

use alloc::{string::String, vec::Vec};

#[derive(Default)]
pub struct Trail {
    left: Vec<String>,
}

impl Trail {
    /// Back went up out of `from`.
    pub fn went_up(&mut self, from: &str) {
        self.left.push(String::from(from));
    }

    /// The folder shown changed by any way but Back or Forward. Opening the
    /// folder Back just left is the same step Forward takes, so the trail
    /// holds; anywhere else ends it.
    pub fn arrived(&mut self, at: &str) {
        if self.left.last().is_some_and(|top| top == at) {
            self.left.pop();
        } else {
            self.left.clear();
        }
    }

    /// The folder Forward opens, taken off the trail.
    pub fn forward(&mut self) -> Option<String> {
        self.left.pop()
    }

    pub fn can_forward(&self) -> bool {
        !self.left.is_empty()
    }
}
