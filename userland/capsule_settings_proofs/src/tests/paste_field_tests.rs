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

//! A pasted line into the search box and the Wi-Fi passphrase.

use nonos_app_skeleton::input::text::first_line;

use super::paste_rig::*;
use crate::text::cache::STRING_CAP;
use crate::text::edit_buffer::EditBuffer;
use crate::text::edit_paste::Pasted;

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
