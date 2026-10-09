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

//! The key repeating, and the press and release that start and end it.

use super::repeats;

#[derive(Clone, Copy)]
pub struct KeyRepeat {
    // The usage repeating, 0 for none.
    pub(super) key: u8,
    // When the key went down, learnt at the first tick after the press.
    pub(super) since_ms: Option<u64>,
    pub(super) next_ms: u64,
}

impl KeyRepeat {
    pub const fn new() -> Self {
        Self { key: 0, since_ms: None, next_ms: 0 }
    }

    /// `key` went down: it repeats from DELAY_MS after the next tick. A key
    /// that does not repeat ends any repeat in progress, as on PS/2.
    pub fn press(&mut self, key: u8) {
        *self = Self::new();
        if repeats(key) {
            self.key = key;
        }
    }

    /// `key` came up: its repeat, if it was the one repeating, ends.
    pub fn release(&mut self, key: u8) {
        if self.key == key {
            *self = Self::new();
        }
    }
}

impl Default for KeyRepeat {
    fn default() -> Self {
        Self::new()
    }
}
