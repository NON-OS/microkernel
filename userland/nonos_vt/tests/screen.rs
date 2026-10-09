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

//! Printing, wrapping and history, against what xterm does for the
//! sequences full-screen tools rely on.

#[path = "support/term.rs"]
mod support;

use support::term;

#[test]
fn the_bottom_right_cell_does_not_scroll() {
    // vim and htop draw their last line into the last cell.
    let t = term(4, 2, "ab\r\nwxyz");
    assert_eq!(t.screen_text(), ["ab", "wxyz"]);
    assert_eq!((t.cursor().x, t.cursor().y), (3, 1));
}

#[test]
fn a_full_line_then_newline_adds_no_blank_line() {
    let t = term(4, 3, "abcd\r\nef");
    assert_eq!(t.screen_text(), ["abcd", "ef", ""]);
}

#[test]
fn printing_past_the_edge_wraps_and_marks_the_line() {
    let t = term(4, 3, "abcdef");
    assert_eq!(t.screen_text(), ["abcd", "ef", ""]);
    assert!(t.visible_line(0).wrapped);
}

#[test]
fn autowrap_off_overwrites_the_last_column() {
    let t = term(4, 2, "\x1b[?7labcdef");
    assert_eq!(t.screen_text(), ["abcf", ""]);
}

#[test]
fn history_keeps_what_scrolls_off() {
    let t = term(4, 2, "a\r\nb\r\nc\r\nd");
    assert_eq!(t.history_len(), 2);
    assert_eq!(t.row_text(0), "c");
}
