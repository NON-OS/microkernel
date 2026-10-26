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

//! The keys scan set 1 sends behind an E0 prefix, which Linux numbers from
//! 96 up, mapped as the PS/2 driver maps their E0 codes.

use super::keycodes::{
    KEYCODE_DEL, KEYCODE_DOWN, KEYCODE_END, KEYCODE_HOME, KEYCODE_INS, KEYCODE_LEFT, KEYCODE_LMETA,
    KEYCODE_MUTE, KEYCODE_PAUSE, KEYCODE_PGDN, KEYCODE_PGUP, KEYCODE_POWER, KEYCODE_PRINT,
    KEYCODE_RALT, KEYCODE_RCTRL, KEYCODE_RIGHT, KEYCODE_RMETA, KEYCODE_UP, KEYCODE_VOLUME_DOWN,
    KEYCODE_VOLUME_UP,
};

pub(super) fn keycode_for(code: u16) -> Option<u32> {
    let v = match code {
        96 => 0x0D,          // KEY_KPENTER
        97 => KEYCODE_RCTRL, // KEY_RIGHTCTRL
        98 => b'/' as u32,   // KEY_KPSLASH
        99 => KEYCODE_PRINT, // KEY_SYSRQ: Print Screen
        100 => KEYCODE_RALT,
        102 => KEYCODE_HOME,
        103 => KEYCODE_UP,
        104 => KEYCODE_PGUP,
        105 => KEYCODE_LEFT,
        106 => KEYCODE_RIGHT,
        107 => KEYCODE_END,
        108 => KEYCODE_DOWN,
        109 => KEYCODE_PGDN,
        110 => KEYCODE_INS,
        111 => KEYCODE_DEL,
        113 => KEYCODE_MUTE,
        114 => KEYCODE_VOLUME_DOWN,
        115 => KEYCODE_VOLUME_UP,
        116 => KEYCODE_POWER,
        119 => KEYCODE_PAUSE,
        125 => KEYCODE_LMETA,
        126 => KEYCODE_RMETA,
        _ => return None,
    };
    Some(v)
}
