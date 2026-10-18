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

//! Which keys type, and which never do.

use super::key::key;
use nonos_app_skeleton::input::text::{text_char, typed_char};
use nonos_app_skeleton::{
    InputKind, KEY_BACKSPACE, KEY_DELETE, KEY_ENTER, KEY_ESC, KEY_F1, KEY_LEFT, KEY_TAB, MOD_ALT,
    MOD_ALTGR, MOD_CAPS, MOD_CTRL, MOD_META, MOD_SHIFT,
};

#[test]
fn any_printable_character_the_keymap_resolves_types_itself() {
    for ch in ['a', 'Z', '~', ' ', 'é', 'ß', 'ø', 'ж', '€', '£', '日', '😀'] {
        assert_eq!(text_char(ch as u32), Some(ch), "{ch}");
        assert_eq!(typed_char(&key(ch as u32, MOD_SHIFT | MOD_CAPS)), Some(ch));
    }
}

#[test]
fn keys_and_control_codes_type_nothing() {
    for code in [KEY_BACKSPACE, KEY_TAB, KEY_ENTER, KEY_ESC, KEY_DELETE, KEY_LEFT, KEY_F1, 0x7F, 0]
    {
        assert_eq!(text_char(code), None, "{code:#x}");
    }
    // The keymap's modifier and function codes share numbers with Hangul
    // Jamo and Ethiopic; a code there is a key.
    for code in [0x1001, 0x100B, 0x1106, 0x1201, 0x120A, 0x12FF] {
        assert_eq!(text_char(code), None, "{code:#x}");
    }
    assert_eq!(text_char(0xD800), None, "a surrogate is not a scalar");
    assert_eq!(text_char(0x11_0000), None);
}

#[test]
fn a_command_chord_is_not_text_but_altgr_is() {
    for m in [MOD_CTRL, MOD_ALT, MOD_META, MOD_CTRL | MOD_SHIFT] {
        assert_eq!(typed_char(&key('v' as u32, m)), None, "{m:#x}");
    }
    assert_eq!(typed_char(&key('€' as u32, MOD_ALTGR)), Some('€'), "a layout's third level");
    let mut up = key('a' as u32, 0);
    up.kind = InputKind::KeyUp;
    assert_eq!(typed_char(&up), None, "only a press types");
}
