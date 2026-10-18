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

//! The keys a client has been told are down. Pure, so the host proofs hold
//! it.
//!
//! The input drivers post a held key's hardware repeats as more presses, but
//! a Wayland client is told only changes and repeats a held key itself, at
//! the rate repeat_info gives. So a press of a key already down is dropped,
//! and so is a release of one that is not.

use alloc::vec::Vec;

/// More keys than any hand holds; a press past it is not sent.
pub const MOST_HELD: usize = 32;

#[derive(Default)]
pub struct HeldKeys {
    down: Vec<u32>,
}

impl HeldKeys {
    pub const fn new() -> HeldKeys {
        HeldKeys { down: Vec::new() }
    }

    /// Whether a press of `key` is news to the client; it is then down.
    pub fn press(&mut self, key: u32) -> bool {
        if self.down.contains(&key) || self.down.len() >= MOST_HELD {
            return false;
        }
        self.down.push(key);
        true
    }

    /// Whether a release of `key` is news to the client; it is then up.
    pub fn release(&mut self, key: u32) -> bool {
        let before = self.down.len();
        self.down.retain(|k| *k != key);
        self.down.len() != before
    }
}
