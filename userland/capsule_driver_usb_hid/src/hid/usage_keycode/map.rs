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

//! The map from a USB keyboard usage onto its keycode.

use super::codes::{
    KEYCODE_CAPS, KEYCODE_DEL, KEYCODE_DOWN, KEYCODE_END, KEYCODE_F1, KEYCODE_F11, KEYCODE_F12,
    KEYCODE_HOME, KEYCODE_INS, KEYCODE_LEFT, KEYCODE_MUTE, KEYCODE_NUMLK, KEYCODE_PGDN,
    KEYCODE_PGUP, KEYCODE_POWER, KEYCODE_RIGHT, KEYCODE_SCROLL, KEYCODE_UP, KEYCODE_VOLUME_DOWN,
    KEYCODE_VOLUME_UP,
};

/// The code for `usage`, or None for a key the layout resolves (letters,
/// digits, symbols, and the control characters Enter, Escape, Backspace and
/// Tab). The keypad posts its own character on every layout, as the PS/2
/// keypad does with Num Lock on.
pub fn usage_keycode(usage: u8) -> Option<u32> {
    let code = match usage {
        0x39 => KEYCODE_CAPS,
        // F1 to F10 run on in both tables; F11 and F12 follow them.
        0x3a..=0x43 => KEYCODE_F1 + u32::from(usage - 0x3a),
        0x44 => KEYCODE_F11,
        0x45 => KEYCODE_F12,
        0x47 => KEYCODE_SCROLL,
        0x49 => KEYCODE_INS,
        0x4a => KEYCODE_HOME,
        0x4b => KEYCODE_PGUP,
        0x4c => KEYCODE_DEL,
        0x4d => KEYCODE_END,
        0x4e => KEYCODE_PGDN,
        0x4f => KEYCODE_RIGHT,
        0x50 => KEYCODE_LEFT,
        0x51 => KEYCODE_DOWN,
        0x52 => KEYCODE_UP,
        0x53 => KEYCODE_NUMLK,
        0x54 => u32::from(b'/'),
        0x55 => u32::from(b'*'),
        0x56 => u32::from(b'-'),
        0x57 => u32::from(b'+'),
        0x58 => 0x0D,
        0x59..=0x61 => u32::from(b'1' + (usage - 0x59)),
        0x62 => u32::from(b'0'),
        0x63 => u32::from(b'.'),
        0x66 => KEYCODE_POWER,
        // The keyboard page's own volume keys. A keyboard that sends its
        // media keys on a consumer control interface instead is not read: the
        // driver binds the boot keyboard alone (descriptors/parse.rs).
        0x7f => KEYCODE_MUTE,
        0x80 => KEYCODE_VOLUME_UP,
        0x81 => KEYCODE_VOLUME_DOWN,
        _ => return None,
    };
    Some(code)
}
