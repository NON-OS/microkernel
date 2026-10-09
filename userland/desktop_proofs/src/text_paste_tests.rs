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

//! Which key pastes, and what the clipboard's text becomes in a one-line
//! field (app_skeleton input/text), the reading every app's paste shares.

use nonos_app_skeleton::input::text::{first_line, is_paste, paste_char, PasteLine};
use nonos_app_skeleton::{
    InputEvent, InputKind, KEY_INSERT, MOD_ALT, MOD_CAPS, MOD_CTRL, MOD_META, MOD_SHIFT,
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
fn ctrl_v_and_shift_insert_paste() {
    assert!(is_paste(&key('v' as u32, MOD_CTRL)));
    assert!(is_paste(&key('V' as u32, MOD_CTRL | MOD_CAPS)));
    assert!(is_paste(&key('V' as u32, MOD_CTRL | MOD_SHIFT)));
    assert!(is_paste(&key(KEY_INSERT, MOD_SHIFT)));
}

#[test]
fn nothing_else_pastes() {
    assert!(!is_paste(&key('v' as u32, 0)), "a plain v is a letter");
    assert!(!is_paste(&key('v' as u32, MOD_CTRL | MOD_ALT)));
    assert!(!is_paste(&key('v' as u32, MOD_META)));
    assert!(!is_paste(&key(KEY_INSERT, 0)), "Insert alone");
    assert!(!is_paste(&key(KEY_INSERT, MOD_CTRL | MOD_SHIFT)));
    let mut up = key('v' as u32, MOD_CTRL);
    up.kind = InputKind::KeyUp;
    assert!(!is_paste(&up), "the release does not paste a second time");
}

fn line(bytes: &[u8]) -> Option<(&str, bool)> {
    first_line(bytes).map(|PasteLine { text, more }| (text, more))
}

#[test]
fn a_field_takes_the_first_line_trimmed() {
    assert_eq!(line(b"hello"), Some(("hello", false)));
    assert_eq!(line(b"  word \n"), Some(("word", false)), "the copied newline is not more");
    assert_eq!(line(b"one\r\ntwo"), Some(("one", true)));
    assert_eq!(line(b"\n\nlater"), Some(("later", false)), "leading blank lines skipped");
    assert_eq!(line(b"caf\xc3\xa9"), Some(("café", false)));
    assert_eq!(line(b""), Some(("", false)), "an empty clipboard is an empty line");
}

#[test]
fn bytes_that_are_not_text_are_refused() {
    assert_eq!(line(b"ab\xff\xfecd"), None);
    assert_eq!(line(b"\xc3("), None);
    // Junk past the first line is never pasted; the line is, and the junk
    // counts as more.
    assert_eq!(line(b"name\n\xff\xfe"), Some(("name", true)));
}

#[test]
fn a_character_cut_by_the_buffer_is_dropped_and_the_rest_kept() {
    // A clipboard longer than the read buffer arrives cut, possibly inside
    // a character; what came before it is good text.
    assert_eq!(line(b"na\xc3\xafve \xe2\x82"), Some(("naïve", false)));
}

#[test]
fn a_tab_is_a_space_and_other_controls_are_nothing() {
    assert_eq!(paste_char('\t'), Some(' '));
    assert_eq!(paste_char('é'), Some('é'));
    for c in ['\u{0}', '\u{7}', '\u{1b}', '\u{7f}', '\u{85}'] {
        assert_eq!(paste_char(c), None, "{c:?}");
    }
}
