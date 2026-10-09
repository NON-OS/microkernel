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

//! Saving the cursor, the alternate screen, tab stops, insert and origin
//! modes.

#[path = "support/term.rs"]
mod support;
use support::term;

#[test]
fn repeat_prints_the_last_character() {
    let t = term(8, 1, "x\x1b[3b");
    assert_eq!(t.row_text(0), "xxxx");
}

#[test]
fn save_and_restore_the_cursor() {
    let t = term(8, 3, "\x1b[2;3H\x1b7\x1b[H\x1b8X");
    assert_eq!(t.row_text(1), "  X");
    let t = term(8, 3, "\x1b[2;3H\x1b[s\x1b[H\x1b[uY");
    assert_eq!(t.row_text(1), "  Y");
}

#[test]
fn alternate_screen_restores_what_was_underneath() {
    let t = term(8, 2, "shell\x1b[?1049hfull\x1b[?1049l");
    assert_eq!(t.screen_text(), ["shell", ""]);
    assert_eq!((t.cursor().x, t.cursor().y), (5, 0));
    assert!(!t.alt_active());
}

#[test]
fn alternate_screen_output_never_reaches_history() {
    let mut t = term(4, 2, "\x1b[?1049h");
    t.feed(b"1\r\n2\r\n3\r\n4\r\n");
    t.feed(b"\x1b[?1049l");
    assert_eq!(t.history_len(), 0);
}

#[test]
fn tab_stops_set_and_cleared() {
    let t = term(20, 1, "\ta");
    assert_eq!((t.cursor().x, t.cursor().y), (9, 0));
    let t = term(20, 1, "\x1b[3g\x1b[1;4H\x1bH\r\tb");
    assert_eq!(t.row_text(0), "   b");
}

#[test]
fn insert_mode_shifts_the_line() {
    let t = term(6, 1, "abc\x1b[1;2H\x1b[4hX");
    assert_eq!(t.row_text(0), "aXbc");
}

#[test]
fn origin_mode_counts_from_the_region() {
    let t = term(4, 4, "\x1b[2;3r\x1b[?6h\x1b[1;1HZ");
    assert_eq!(t.row_text(1), "Z");
}
