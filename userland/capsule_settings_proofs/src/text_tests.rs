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

use crate::text::cache::STRING_CAP;
use crate::text::edit_buffer::EditBuffer;
use crate::text::valbuf::{ValBuf, CAP};

fn held(b: &EditBuffer) -> &str {
    core::str::from_utf8(b.as_slice()).expect("the buffer is always UTF-8")
}

fn typed(s: &str) -> EditBuffer {
    let mut b = EditBuffer::empty();
    for ch in s.chars() {
        assert!(b.push_char(ch, STRING_CAP));
    }
    b
}

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

#[test]
fn a_query_written_into_a_full_heading_never_cuts_a_character() {
    let mut v = ValBuf::new();
    v.push_str(&"a".repeat(CAP - 1));
    v.push_str("é");
    assert_eq!(v.as_str(), "a".repeat(CAP - 1), "the two byte e does not fit in one");
    let mut v = ValBuf::new();
    v.push_str("No setting matches \"Größe\".");
    assert_eq!(v.as_str(), "No setting matches \"Größe\".");
}

/* The Wi-Fi rows format numbers and ids at paint time without allocating. */
#[test]
fn decimal_and_hex_go_in_as_digits() {
    let mut b = ValBuf::new();
    b.push_dec(0);
    b.push_str(" ");
    b.push_dec(4_294_967_295);
    b.push_str(" ");
    b.push_hex16(0x10EC);
    b.push_str(":");
    b.push_hex16(0xC821);
    assert_eq!(b.as_str(), "0 4294967295 10ec:c821");
}

#[test]
fn the_longest_no_driver_line_fits_the_row() {
    let mut b = ValBuf::new();
    b.push_str("Wi-Fi chip ");
    b.push_hex16(0x14C3);
    b.push_str(":");
    b.push_hex16(0x0616);
    b.push_str(" has no NONOS driver; use Ethernet or USB Wi-Fi");
    assert!(b.as_str().ends_with("USB Wi-Fi"), "{}", b.as_str());
    assert!(b.as_str().len() <= CAP);
}
