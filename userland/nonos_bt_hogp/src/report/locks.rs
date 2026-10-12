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
//! The lock keys and the LEDs that show them. The boot keyboard output
//! report is one octet (HID 1.11, B.1): bit 0 Num Lock, bit 1 Caps Lock,
//! bit 2 Scroll Lock, bit 3 Compose, bit 4 Kana. The host owns the lock
//! state, as with a USB keyboard: a lock changes when its key goes down.

use super::input::Input;

pub const NUM_LOCK: u8 = 1 << 0;
pub const CAPS_LOCK: u8 = 1 << 1;
pub const SCROLL_LOCK: u8 = 1 << 2;
/// The five bits the boot report defines; the other three are padding.
const DEFINED: u8 = 0x1f;

/// Each lock key and its bit. The usages are those of the Keyboard/Keypad
/// page (HID Usage Tables 1.4, 10).
const KEYS: [(u8, u8); 3] = [(0x53, NUM_LOCK), (0x39, CAPS_LOCK), (0x47, SCROLL_LOCK)];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Locks(u8);

impl Locks {
    pub fn bits(self) -> u8 {
        self.0
    }

    /// Each lock key pressed among the events toggles its lock; whether
    /// any changed. A key held down does not repeat in a boot report, so a
    /// press is counted once.
    pub fn on(&mut self, events: &[Input]) -> bool {
        let was = self.0;
        for e in events {
            if let Input::KeyDown(k) = e {
                if let Some(&(_, bit)) = KEYS.iter().find(|(usage, _)| usage == k) {
                    self.0 ^= bit;
                }
            }
        }
        self.0 != was
    }

    /// The state given from outside, padding cleared; whether it changed.
    pub fn set(&mut self, bits: u8) -> bool {
        let was = self.0;
        self.0 = bits & DEFINED;
        self.0 != was
    }
}
