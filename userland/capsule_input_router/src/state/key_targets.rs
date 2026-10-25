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

// Which capsule received the key-down for each held key, so the key-up goes
// there even if focus moved meanwhile; else the original app sees a stuck
// key. A key is its source's: a press on one keyboard is never
// released, nor taken over, by the same code on another device.

const MAX_HELD_KEYS: usize = 16;

/// A held key: the kernel's source index for the device, and the code.
pub type Key = (u8, u32);
/// The key an event names: its source from bits 8 to 13 of the flags,
/// which the kernel writes; bits 14 and 15 are a Bluetooth bond's marks.
pub const fn key_of(flags: u16, code: u32) -> Key {
    (((flags >> 8) & 0x3F) as u8, code)
}

/// A held key and the pid its press went to; pid 0 is a free slot.
type Held = (Key, u32);
const FREE: Held = ((0, 0), 0);

pub struct KeyTargets {
    held: [Held; MAX_HELD_KEYS],
}

impl KeyTargets {
    pub const fn new() -> Self {
        Self { held: [FREE; MAX_HELD_KEYS] }
    }

    // `pid` received the down for `key`: its entry, else a free slot; a full
    // table drops the record (that key-up then falls back to focus).
    pub fn remember(&mut self, key: Key, pid: u32) {
        if let Some(slot) = self.held.iter_mut().find(|h| h.1 != 0 && h.0 == key) {
            slot.1 = pid;
        } else if let Some(slot) = self.held.iter_mut().find(|h| h.1 == 0) {
            *slot = (key, pid);
        }
    }

    // The pid holding `key` down, if any, left in place.
    pub fn held_by(&self, key: Key) -> Option<u32> {
        self.held.iter().find(|h| h.1 != 0 && h.0 == key).map(|h| h.1)
    }

    // Return and clear the pid that received the down for `key`, if known.
    pub fn take(&mut self, key: Key) -> Option<u32> {
        let slot = self.held.iter_mut().find(|h| h.1 != 0 && h.0 == key)?;
        Some(core::mem::replace(slot, FREE).1)
    }

    // Drop every entry for a pid that has gone away.
    pub fn forget_pid(&mut self, pid: u32) {
        self.held.iter_mut().filter(|h| h.1 == pid).for_each(|h| *h = FREE);
    }
}

impl Default for KeyTargets {
    fn default() -> Self {
        Self::new()
    }
}
