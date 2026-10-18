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

//! Ctrl+V into the rename box or the Launchpad search takes the clipboard's
//! first line, whole characters only, within the field's limit.

use crate::paste_line::paste_line;

#[test]
fn pasted_text_is_appended() {
    let mut f = String::from("report ");
    assert!(paste_line(&mut f, b"2026.txt", 64));
    assert_eq!(f, "report 2026.txt");
}

#[test]
fn only_the_first_line_is_taken() {
    let mut f = String::new();
    assert!(paste_line(&mut f, b"first\nsecond", 64));
    assert_eq!(f, "first");
    let mut g = String::new();
    paste_line(&mut g, b"dos\r\nline", 64);
    assert_eq!(g, "dos");
}

#[test]
fn control_characters_are_dropped() {
    let mut f = String::new();
    paste_line(&mut f, b"a\tb\x1bc\x00d", 64);
    assert_eq!(f, "abcd");
}

#[test]
fn the_field_limit_holds() {
    let mut f = String::from("abc");
    assert!(paste_line(&mut f, b"defghij", 6));
    assert_eq!(f, "abcdef");
    assert!(!paste_line(&mut f, b"x", 6));
    assert_eq!(f, "abcdef");
}

#[test]
fn a_character_is_never_split() {
    let mut f = String::from("ab");
    // "e" with an acute accent is two bytes; one byte of room keeps it out.
    assert!(!paste_line(&mut f, "\u{e9}".as_bytes(), 3));
    assert_eq!(f, "ab");
    // A buffer cut mid-character keeps what came before the cut.
    let cut = &"x\u{e9}".as_bytes()[..2];
    assert!(paste_line(&mut f, cut, 64));
    assert_eq!(f, "abx");
}

#[test]
fn an_empty_or_bad_clipboard_changes_nothing() {
    let mut f = String::from("keep");
    assert!(!paste_line(&mut f, b"", 64));
    assert!(!paste_line(&mut f, b"\n", 64));
    assert!(!paste_line(&mut f, &[0xFF, 0xFE], 64));
    assert_eq!(f, "keep");
}
