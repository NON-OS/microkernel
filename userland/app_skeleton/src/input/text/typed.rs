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

use core::ops::RangeInclusive;

use crate::input::{InputEvent, MOD_ALT, MOD_CTRL, MOD_META};

/// The keymap's own keys: modifiers, function keys, arrows and the editing
/// block (capsule_driver_ps2_input keymap/set1/keycodes.rs). They share
/// numbers with real characters (Hangul Jamo and Ethiopic), so a code in
/// this range is a key, never text.
const KEYMAP_KEYS: RangeInclusive<u32> = 0x1000..=0x12FF;

/// The character a key code types, if it types one: any Unicode scalar the
/// keymap resolved that is not a control character or one of its own keys.
/// The keymap resolves layout, Shift, Caps and AltGr before the app sees the
/// code, so an e acute or a euro sign arrives here as itself.
pub fn text_char(code: u32) -> Option<char> {
    if KEYMAP_KEYS.contains(&code) {
        return None;
    }
    char::from_u32(code).filter(|ch| !ch.is_control())
}

/// The character a key press types into a text field. A chord with Ctrl,
/// Alt or Meta is a command, not text: Ctrl+V used to type a v. AltGr is
/// how a layout reaches its third level, so it still types.
pub fn typed_char(event: &InputEvent) -> Option<char> {
    if !event.is_key_down() || event.flags & (MOD_CTRL | MOD_ALT | MOD_META) != 0 {
        return None;
    }
    text_char(event.code)
}
