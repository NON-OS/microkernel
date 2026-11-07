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
//! The input drivers post a held key's repeats as more presses. A client is
//! told a press only for a key that is up, and a release only for one that
//! is down; a repeat is told as both (input/key.rs), since the client is
//! told not to repeat keys itself (keymap.rs).

use alloc::vec::Vec;

/// More keys than any hand holds; a press past it is not sent.
pub const MOST_HELD: usize = 32;

#[derive(Default)]
pub struct HeldKeys {
    down: Vec<u32>,
    /// The (depressed, locked) masks the client was last told.
    mods: (u32, u32),
}

impl HeldKeys {
    pub const fn new() -> HeldKeys {
        HeldKeys { down: Vec::new(), mods: (0, 0) }
    }

    /// Whether a press of `key` is news to the client; it is then down.
    pub fn press(&mut self, key: u32) -> bool {
        if self.down.contains(&key) || self.down.len() >= MOST_HELD {
            return false;
        }
        self.down.push(key);
        true
    }

    /// Whether the client was told `key` is down.
    pub fn is_down(&self, key: u32) -> bool {
        self.down.contains(&key)
    }

    /// Whether a release of `key` is news to the client; it is then up.
    pub fn release(&mut self, key: u32) -> bool {
        let before = self.down.len();
        self.down.retain(|k| *k != key);
        self.down.len() != before
    }

    /// Whether `masks` are news to the client; they are then what it knows.
    pub fn mods(&mut self, masks: (u32, u32)) -> bool {
        core::mem::replace(&mut self.mods, masks) != masks
    }
}
