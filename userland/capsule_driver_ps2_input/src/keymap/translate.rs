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
use super::flags::{FLAG_BREAK, FLAG_E0_PREFIX, FLAG_E1_PREFIX};
use super::keypad::is_keypad;
use super::set1::{keycode_for as base_keycode_for, KEYCODE_PAUSE};
use super::set1_e0::keycode_for as e0_keycode_for;

/// The scan code that ends either half of the Pause sequence (poll/prefix.rs).
const SCAN_PAUSE: u8 = 0x45;
pub struct Translated {
    pub keycode: u32,
    pub is_release: bool,
    /// A numeric keypad key, which no layout remaps.
    pub keypad: bool,
}
pub fn translate(scancode: u8, flags: u8) -> Option<Translated> {
    let is_release = (flags & FLAG_BREAK) != 0;
    let key = scancode & 0x7F;
    let e0 = (flags & FLAG_E0_PREFIX) != 0;
    let keycode = if flags & FLAG_E1_PREFIX != 0 {
        (key == SCAN_PAUSE).then_some(KEYCODE_PAUSE)?
    } else if e0 {
        e0_keycode_for(key)?
    } else {
        base_keycode_for(key)?
    };
    Some(Translated { keycode, is_release, keypad: is_keypad(key, e0) })
}
