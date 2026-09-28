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

//! A key press as the window reports it, as the key a terminal program
//! reads.

use nonos_app_skeleton::{
    InputEvent, KEY_BACKSPACE, KEY_DELETE, KEY_DOWN, KEY_END, KEY_ENTER, KEY_ESC, KEY_F1, KEY_F12,
    KEY_HOME, KEY_INSERT, KEY_LEFT, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_RIGHT, KEY_TAB, KEY_UP,
    MOD_ALT, MOD_CTRL, MOD_SHIFT,
};
use nonos_vt::input::{Key, Mods};

pub fn mods(flags: u16) -> Mods {
    Mods { shift: flags & MOD_SHIFT != 0, alt: flags & MOD_ALT != 0, ctrl: flags & MOD_CTRL != 0 }
}

pub fn key_of(event: &InputEvent) -> Option<(Key, Mods)> {
    let m = mods(event.flags);
    let key = match event.code {
        KEY_ENTER => Key::Enter,
        KEY_TAB => Key::Tab,
        KEY_BACKSPACE | 0x7F => Key::Backspace,
        KEY_ESC => Key::Escape,
        KEY_UP => Key::Up,
        KEY_DOWN => Key::Down,
        KEY_LEFT => Key::Left,
        KEY_RIGHT => Key::Right,
        KEY_HOME => Key::Home,
        KEY_END => Key::End,
        KEY_PAGE_UP => Key::PageUp,
        KEY_PAGE_DOWN => Key::PageDown,
        KEY_INSERT => Key::Insert,
        KEY_DELETE => Key::Delete,
        c @ KEY_F1..=KEY_F12 => Key::F((c - KEY_F1 + 1) as u8),
        c => Key::Char(char::from_u32(c).filter(|ch| !ch.is_control())?),
    };
    Some((key, m))
}
