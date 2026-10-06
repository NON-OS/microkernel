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

//! Wide and combining characters, and line drawing.

#[path = "support/term.rs"]
mod support;
use support::term;

#[test]
fn wide_characters_take_two_cells_and_wrap_whole() {
    let t = term(5, 2, "ab漢字");
    assert_eq!(t.screen_text(), ["ab漢", "字"]);
    assert!(t.visible_line(0).cell(2).is_wide());
    assert!(t.visible_line(0).cell(3).is_tail());
}

#[test]
fn overwriting_half_a_wide_character_blanks_the_other_half() {
    let t = term(4, 1, "漢\x1b[1;2Hx");
    let line = t.visible_line(0);
    assert_eq!(line.cell(0).ch, ' ');
    assert!(!line.cell(0).is_wide());
    assert_eq!(line.cell(1).ch, 'x');
}

#[test]
fn combining_marks_join_the_character_before() {
    let t = term(4, 1, "e\u{301}x");
    assert_eq!(t.row_text(0), "e\u{301}x");
    assert_eq!((t.cursor().x, t.cursor().y), (2, 0));
}

#[test]
fn emoji_joined_by_zwj_stay_in_one_cell_pair() {
    let t = term(6, 1, "👩\u{200d}💻!");
    assert_eq!(t.visible_line(0).cell(2).ch, '!');
}

#[test]
fn dec_line_drawing() {
    let t = term(4, 1, "\x1b(0lqk\x1b(Bq");
    assert_eq!(t.row_text(0), "┌─┐q");
}
