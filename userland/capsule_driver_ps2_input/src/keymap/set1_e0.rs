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
    KEYCODE_DEL, KEYCODE_DOWN, KEYCODE_END, KEYCODE_HOME, KEYCODE_INS, KEYCODE_LEFT, KEYCODE_LMETA,
    KEYCODE_MUTE, KEYCODE_PGDN, KEYCODE_PGUP, KEYCODE_POWER, KEYCODE_RALT, KEYCODE_RCTRL,
    KEYCODE_RIGHT, KEYCODE_RMETA, KEYCODE_UP, KEYCODE_VOLUME_DOWN, KEYCODE_VOLUME_UP,
};
pub fn keycode_for(scan: u8) -> Option<u32> {
    let v = match scan {
        0x1C => 0x0D,
        0x1D => KEYCODE_RCTRL,
        // A laptop's Fn volume keys come through its i8042 as these (an HP
        // 15s among them): the codes of the Windows multimedia keyboards the
        // firmware imitates.
        0x20 => KEYCODE_MUTE,
        0x2E => KEYCODE_VOLUME_DOWN,
        0x30 => KEYCODE_VOLUME_UP,
        0x35 => b'/' as u32,
        0x38 => KEYCODE_RALT,
        0x47 => KEYCODE_HOME,
        0x48 => KEYCODE_UP,
        0x49 => KEYCODE_PGUP,
        0x4B => KEYCODE_LEFT,
        0x4D => KEYCODE_RIGHT,
        0x4F => KEYCODE_END,
        0x50 => KEYCODE_DOWN,
        0x51 => KEYCODE_PGDN,
        0x52 => KEYCODE_INS,
        0x53 => KEYCODE_DEL,
        0x5B => KEYCODE_LMETA,
        0x5C => KEYCODE_RMETA,
        // ACPI Power, from a keyboard with a power key of its own.
        0x5E => KEYCODE_POWER,
        _ => return None,
    };
    Some(v)
}
