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

//! Which rows a resize keeps on screen.

#[path = "support/term.rs"]
mod support;
use support::term;

#[test]
fn lines_pushed_off_the_top_go_to_history() {
    let mut t = term(4, 2, "a\r\nb");
    t.resize(4, 1);
    assert_eq!(t.screen_text(), ["b"]);
    assert_eq!(t.history_len(), 1);
}

#[test]
fn growing_at_the_foot_brings_history_back() {
    let mut t = term(4, 2, "a\r\nb\r\nc");
    assert_eq!(t.history_len(), 1);
    t.resize(4, 3);
    assert_eq!(t.screen_text(), ["a", "b", "c"]);
    assert_eq!((t.cursor().x, t.cursor().y), (1, 2));
}

#[test]
fn growing_mid_screen_opens_rows_below() {
    let mut t = term(4, 3, "a\r\nb\r\nc\r\nd\x1b[1;1H");
    t.resize(4, 5);
    assert_eq!(t.screen_text(), ["b", "c", "d", "", ""]);
}
