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

//! Proofs for how far the code view may scroll: the rows a buffer wraps
//! into, and the clamp that keeps the last of them on screen after an edit
//! shortens the text.

use crate::clamp_scroll::clamp_scroll;
use crate::edit_tests::doc;
use crate::max_scroll::max_scroll;
use crate::mode::Mode;
use crate::visual_lines::visual_lines;

#[test]
fn rows_count_newlines_and_wraps_at_the_column_limit() {
    assert_eq!(visual_lines(b"", 80), 1, "an empty buffer is still one row");
    assert_eq!(visual_lines(b"one\ntwo\n", 80), 3, "a trailing newline opens a row");
    assert_eq!(visual_lines(b"abcdefgh", 4), 2, "eight columns at four wrap once");
    assert_eq!(visual_lines(b"abcdefghi", 4), 3);
    assert_eq!(visual_lines(b"abcd\nefgh", 4), 2, "a newline at the limit wraps once, not twice");
}

#[test]
fn rows_count_characters_not_bytes() {
    let wide = "\u{e9}\u{e9}\u{e9}\u{e9}";
    assert_eq!(wide.len(), 8);
    assert_eq!(visual_lines(wide.as_bytes(), 4), 1);
}

#[test]
fn the_last_scroll_shows_the_last_page() {
    assert_eq!(max_scroll(100, 30), 70);
    assert_eq!(max_scroll(10, 30), 0, "a buffer shorter than the view does not scroll");
}

#[test]
fn a_scroll_past_the_end_comes_back_to_the_last_page() {
    let mut s = doc("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
    s.mode = Mode::Code;
    s.scroll_line = 50;
    clamp_scroll(&mut s, 4);
    assert_eq!(s.scroll_line, 6);

    s.scroll_line = 2;
    clamp_scroll(&mut s, 4);
    assert_eq!(s.scroll_line, 2, "a scroll inside the buffer is left alone");
}

#[test]
fn the_document_view_keeps_its_own_scroll() {
    let mut s = doc("short");
    s.mode = Mode::Document;
    s.scroll_line = 50;
    clamp_scroll(&mut s, 4);
    assert_eq!(s.scroll_line, 50);
}
