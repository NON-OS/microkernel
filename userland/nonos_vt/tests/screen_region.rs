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

//! The scroll region, line insertion and deletion, and erasing.

#[path = "support/term.rs"]
mod support;
use support::term;

#[test]
fn a_scroll_region_keeps_the_status_line() {
    let t = term(6, 4, "\x1b[4;1Hstatus\x1b[1;3r\x1b[1;1Ha\r\nb\r\nc\r\nd");
    assert_eq!(t.screen_text(), ["b", "c", "d", "status"]);
}

#[test]
fn reverse_index_at_the_top_scrolls_down() {
    let t = term(4, 3, "one\r\ntwo\x1b[H\x1bMnew");
    assert_eq!(t.screen_text(), ["new", "one", "two"]);
}

#[test]
fn insert_and_delete_lines_stay_inside_the_region() {
    let t = term(4, 4, "a\r\nb\r\nc\r\nd\x1b[1;3r\x1b[2;1H\x1b[L");
    assert_eq!(t.screen_text(), ["a", "", "b", "d"]);
    let t = term(4, 4, "a\r\nb\r\nc\r\nd\x1b[1;3r\x1b[1;1H\x1b[M");
    assert_eq!(t.screen_text(), ["b", "c", "", "d"]);
}

#[test]
fn scroll_down_and_erase_chars() {
    let t = term(4, 3, "a\r\nb\x1b[T");
    assert_eq!(t.screen_text(), ["", "a", "b"]);
    let t = term(6, 1, "abcdef\x1b[1;2H\x1b[3X");
    assert_eq!(t.screen_text(), ["a   ef"]);
    assert_eq!((t.cursor().x, t.cursor().y), (1, 0));
}

#[test]
fn erase_to_start_includes_the_cursor_cell() {
    let t = term(5, 1, "abcde\x1b[1;3H\x1b[1K");
    assert_eq!(t.row_text(0), "   de");
}

#[test]
fn erase_display_leaves_the_cursor() {
    let t = term(5, 2, "ab\r\ncd\x1b[2J");
    assert_eq!(t.screen_text(), ["", ""]);
    assert_eq!((t.cursor().x, t.cursor().y), (2, 1));
}

#[test]
fn a_huge_scroll_count_is_bounded() {
    let t = term(4, 3, "a\x1b[65535S");
    assert_eq!(t.screen_text(), ["", "", ""]);
}
