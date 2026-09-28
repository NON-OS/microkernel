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

//! Resizing re-wraps text rather than cutting it.

#[path = "support/term.rs"]
mod support;

use support::term;

#[test]
fn narrowing_wraps_and_widening_joins_again() {
    let mut t = term(10, 3, "abcdefgh\r\nxy");
    t.resize(4, 4);
    assert_eq!(t.screen_text(), ["abcd", "efgh", "xy", ""]);
    t.resize(10, 3);
    assert_eq!(t.screen_text(), ["abcdefgh", "xy", ""]);
}

#[test]
fn the_cursor_keeps_its_place_in_the_text() {
    let mut t = term(10, 3, "abcdefgh");
    assert_eq!((t.cursor().x, t.cursor().y), (8, 0));
    t.resize(4, 3);
    assert_eq!((t.cursor().x, t.cursor().y), (3, 1));
    t.feed(b"Z");
    assert_eq!(t.screen_text(), ["abcd", "efgh", "Z"]);
}

#[test]
fn a_wide_character_is_never_split() {
    let mut t = term(6, 2, "abc漢");
    t.resize(4, 3);
    assert_eq!(t.screen_text(), ["abc", "漢", ""]);
}

#[test]
fn the_alternate_screen_is_cut_not_rewrapped() {
    let mut t = term(8, 2, "\x1b[?1049habcdefgh");
    t.resize(4, 2);
    assert_eq!(t.screen_text(), ["abcd", ""]);
}
