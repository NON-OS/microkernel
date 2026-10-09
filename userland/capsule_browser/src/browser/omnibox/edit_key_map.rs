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

use nonos_app_skeleton::{
    KEY_BACKSPACE, KEY_DELETE, KEY_END, KEY_ENTER, KEY_ESC, KEY_F6, KEY_HOME, KEY_INSERT, KEY_LEFT,
    KEY_RIGHT, MOD_ALT, MOD_CTRL, MOD_SHIFT,
};

use super::edit_key::EditKey;
use super::text_char::text_char;

/* Map a key-down to its editing action, the way mainstream browsers bind
 * them: Ctrl or Alt with Backspace and Ctrl with Delete or an arrow act on
 * words, Shift extends the selection, Shift+Delete and Shift+Insert are the
 * old cut and paste pair, and a letter with Ctrl is a shortcut, never text. */
pub fn edit_key(code: u32, flags: u16) -> EditKey {
    let ctrl = flags & MOD_CTRL != 0;
    let alt = flags & MOD_ALT != 0;
    let shift = flags & MOD_SHIFT != 0;
    match code {
        KEY_ENTER => EditKey::Commit,
        KEY_ESC => EditKey::Cancel,
        KEY_F6 => EditKey::FocusOmnibox,
        KEY_BACKSPACE if ctrl || alt => EditKey::WordBackspace,
        KEY_BACKSPACE => EditKey::Backspace,
        KEY_DELETE if shift && !ctrl => EditKey::Cut,
        KEY_DELETE if ctrl => EditKey::WordDelete,
        KEY_DELETE => EditKey::Delete,
        KEY_INSERT if shift => EditKey::Paste,
        KEY_INSERT if ctrl => EditKey::Copy,
        KEY_LEFT => EditKey::Left { word: ctrl, extend: shift },
        KEY_RIGHT => EditKey::Right { word: ctrl, extend: shift },
        KEY_HOME => EditKey::Home { extend: shift },
        KEY_END => EditKey::End { extend: shift },
        _ if ctrl && !alt => shortcut(code),
        _ if alt && !ctrl && lower(code) == Some('d') => EditKey::FocusOmnibox,
        _ => match text_char(code, flags) {
            Some(c) => EditKey::Insert(c),
            None => EditKey::Ignore,
        },
    }
}

fn shortcut(code: u32) -> EditKey {
    match lower(code) {
        Some('a') => EditKey::SelectAll,
        Some('c') => EditKey::Copy,
        Some('x') => EditKey::Cut,
        Some('v') => EditKey::Paste,
        Some('l') => EditKey::FocusOmnibox,
        _ => EditKey::Ignore,
    }
}

fn lower(code: u32) -> Option<char> {
    char::from_u32(code).map(|c| c.to_ascii_lowercase())
}
