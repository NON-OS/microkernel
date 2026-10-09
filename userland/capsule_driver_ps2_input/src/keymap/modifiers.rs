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

use super::set1::{
    KEYCODE_LALT, KEYCODE_LCTRL, KEYCODE_LMETA, KEYCODE_LSHIFT, KEYCODE_RALT, KEYCODE_RCTRL,
    KEYCODE_RMETA, KEYCODE_RSHIFT,
};

// Modifier flag bits, matching the app-side MOD_* contract and the USB HID
// driver's encoding (shift, ctrl, alt, meta, caps).
pub const MOD_SHIFT: u16 = 1 << 0;
pub const MOD_CTRL: u16 = 1 << 1;
pub const MOD_ALT: u16 = 1 << 2;
pub const MOD_META: u16 = 1 << 3;
pub const MOD_CAPS: u16 = 1 << 4;
pub const MOD_ALTGR: u16 = 1 << 6;

/// The modifier bit a keycode toggles, or None for a normal key.
pub fn modifier_bit(keycode: u32) -> Option<u16> {
    match keycode {
        KEYCODE_LSHIFT | KEYCODE_RSHIFT => Some(MOD_SHIFT),
        KEYCODE_LCTRL | KEYCODE_RCTRL => Some(MOD_CTRL),
        KEYCODE_LALT => Some(MOD_ALT),
        // Right alt is AltGr on every layout here but US, where nothing is
        // mapped to it, so it never doubles as alt.
        KEYCODE_RALT => Some(MOD_ALTGR),
        KEYCODE_LMETA | KEYCODE_RMETA => Some(MOD_META),
        _ => None,
    }
}
