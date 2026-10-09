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

//! A pasted line into the panel's fields: the search box, a stored value
//! (whose rule is the policy store's) and the Wi-Fi passphrase.

use nonos_app_skeleton::input::text::first_line;

use crate::text::cache::STRING_CAP;
use crate::text::edit_buffer::EditBuffer;
use crate::text::edit_paste::{value_char, Pasted};

fn held(b: &EditBuffer) -> &str {
    core::str::from_utf8(b.as_slice()).expect("UTF-8")
}

#[test]
fn a_hostname_pastes_whole_and_the_clipboards_newline_stays_behind() {
    let clip = first_line(b"nonos-box.lan\n").expect("text");
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste(clip.text, 63, usize::MAX, value_char), Pasted::Whole);
    assert_eq!(held(&b), "nonos-box.lan");
}

#[test]
fn a_value_with_a_character_the_store_refuses_is_refused_whole() {
    let mut b = EditBuffer::empty();
    b.push(b'x');
    for bad in ["my host", "naïve", "a/b", "rm;ls"] {
        assert_eq!(b.paste(bad, 63, usize::MAX, value_char), Pasted::Refused, "{bad}");
        assert_eq!(held(&b), "x", "nothing of {bad} went in");
    }
}

#[test]
fn a_value_longer_than_its_field_is_cut_to_it() {
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste(&"a".repeat(40), 32, usize::MAX, value_char), Pasted::Cut);
    assert_eq!(b.len, 32);
}

#[test]
fn the_search_box_takes_any_text_up_to_its_characters() {
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste("Größe\tFont", STRING_CAP, 24, |_| true), Pasted::Whole);
    assert_eq!(held(&b), "Größe Font", "a tab is a space");
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste(&"é".repeat(30), STRING_CAP, 24, |_| true), Pasted::Cut);
    assert_eq!(b.char_count(), 24);
}

#[test]
fn a_passphrase_is_printable_ascii_or_nothing() {
    let ascii = |c: char| matches!(c, ' '..='~');
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste("correct horse #9!", STRING_CAP, usize::MAX, ascii), Pasted::Whole);
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste("pässwort", STRING_CAP, usize::MAX, ascii), Pasted::Refused);
    assert_eq!(b.len, 0);
}

#[test]
fn nothing_printable_is_nothing_to_paste() {
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste("", STRING_CAP, 24, |_| true), Pasted::Empty);
    assert_eq!(b.paste("\u{7}\u{1b}", STRING_CAP, 24, |_| true), Pasted::Empty);
    assert!(first_line(b"\xff\xfe binary").is_none(), "not text, so not pasted at all");
}
