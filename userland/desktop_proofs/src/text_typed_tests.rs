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

//! Which keys type which characters into an app's text field, and the UTF-8
//! edits a fixed buffer field makes (app_skeleton input/text).

use nonos_app_skeleton::input::text::{last_char_len, push_char, text_char, typed_char};
use nonos_app_skeleton::{
    InputEvent, InputKind, KEY_BACKSPACE, KEY_DELETE, KEY_ENTER, KEY_ESC, KEY_F1, KEY_LEFT,
    KEY_TAB, MOD_ALT, MOD_ALTGR, MOD_CAPS, MOD_CTRL, MOD_META, MOD_SHIFT,
};

fn key(code: u32, flags: u16) -> InputEvent {
    InputEvent {
        kind: InputKind::KeyDown,
        flags,
        code,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    }
}

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

#[test]
fn a_fixed_buffer_takes_and_gives_back_whole_characters() {
    let mut buf = [0u8; 8];
    let mut len = 0;
    for ch in ['a', 'é', '€'] {
        len = push_char(&mut buf, len, 8, ch).expect("room");
    }
    assert_eq!(&buf[..len], "aé€".as_bytes());
    assert_eq!(push_char(&mut buf, len, 8, '日'), None, "three bytes into two");
    assert_eq!(push_char(&mut buf, len, 6, 'x'), None, "the cap is the field's, not the array's");
    assert_eq!(last_char_len(&buf[..len]), 3);
    len -= 3;
    assert_eq!(last_char_len(&buf[..len]), 2);
    len -= 2;
    assert_eq!(last_char_len(&buf[..len]), 1);
    assert_eq!(last_char_len(&[]), 0);
}
