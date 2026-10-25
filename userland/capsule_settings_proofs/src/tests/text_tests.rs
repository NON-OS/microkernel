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

//! The panel's text buffers hold UTF-8 and edit whole characters.

use super::text_rig::*;
use crate::text::edit_buffer::EditBuffer;

#[test]
fn typed_characters_go_in_whole_and_count_as_one_each() {
    let b = typed("Größe ünd €");
    assert_eq!(held(&b), "Größe ünd €");
    assert_eq!(b.char_count(), 11);
    assert!(b.len > 11);
}

#[test]
fn backspace_takes_a_whole_character() {
    let mut b = typed("añ€");
    assert!(b.pop());
    assert_eq!(held(&b), "añ");
    assert!(b.pop());
    assert_eq!(held(&b), "a");
    assert!(b.pop());
    assert!(!b.pop(), "nothing left");
    assert_eq!(b.len, 0);
}

#[test]
fn a_character_that_does_not_fit_whole_is_refused() {
    let mut b = EditBuffer::empty();
    for _ in 0..4 {
        b.push(b'a');
    }
    assert!(!b.push_char('€', 6), "three bytes into two");
    assert!(b.push_char('é', 6));
    assert_eq!(held(&b), "aaaaé");
    assert!(!b.push_char('x', 6));
}

#[test]
fn ascii_backspace_is_unchanged() {
    let mut b = typed("abc");
    b.pop();
    assert_eq!(held(&b), "ab");
}
