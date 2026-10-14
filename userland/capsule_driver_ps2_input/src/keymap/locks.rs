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

//! The three lock keys as the keyboard's LED byte (command 0xED): bit 0
//! Scroll Lock, bit 1 Num Lock, bit 2 Caps Lock. Caps Lock also decides the
//! case of every letter; the other two are shown, as a keyboard shows them,
//! and the keypad types its digits either way (keypad.rs).

use super::set1::{KEYCODE_CAPS, KEYCODE_NUMLK, KEYCODE_SCROLL};

pub const LED_SCROLL: u8 = 1 << 0;
pub const LED_NUM: u8 = 1 << 1;
pub const LED_CAPS: u8 = 1 << 2;

/// The LED bit a press of `keycode` toggles, or None for any other key.
pub fn lock_bit(keycode: u32) -> Option<u8> {
    match keycode {
        KEYCODE_CAPS => Some(LED_CAPS),
        KEYCODE_NUMLK => Some(LED_NUM),
        KEYCODE_SCROLL => Some(LED_SCROLL),
        _ => None,
    }
}
