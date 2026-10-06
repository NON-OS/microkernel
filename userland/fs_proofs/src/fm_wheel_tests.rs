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

//! The file manager's wheel: three rows a notch in the list, one line of
//! icons in the grid, three lines in an opened file; a notch away from the
//! user (+1) goes up, and no view scrolls past its content.

use crate::fm_logic::wheel::{listing, preview};

#[test]
fn the_list_moves_three_rows_a_notch() {
    assert_eq!(listing(0, 50, 10, 1, -1), 3);
    assert_eq!(listing(9, 50, 10, 1, 1), 6);
    assert_eq!(listing(39, 50, 10, 1, -1), 40, "the last row stops at the bottom");
    assert_eq!(listing(2, 50, 10, 1, 1), 0);
    assert_eq!(listing(0, 8, 10, 1, -1), 0, "a listing that fits does not move");
}

#[test]
fn the_grid_moves_a_whole_line_of_icons_a_notch() {
    // Five columns, three lines shown, 40 entries: eight lines.
    assert_eq!(listing(0, 40, 15, 5, -1), 5, "one notch, one line");
    assert_eq!(listing(10, 40, 15, 5, 1), 5);
    assert_eq!(listing(25, 40, 15, 5, -1), 25, "five lines down is the end");
    // The keyboard can leave the start mid-line; the wheel puts it on one.
    assert_eq!(listing(7, 40, 15, 5, -1), 10);
}

#[test]
fn an_opened_file_scrolls_three_lines_a_notch_within_its_text() {
    assert_eq!(preview(0, 100, 10, -1), 3);
    assert_eq!(preview(50, 100, 10, 2), 44);
    assert_eq!(preview(89, 100, 10, -1), 90);
    assert_eq!(preview(1, 100, 10, 1), 0);
    assert_eq!(preview(0, 4, 10, -1), 0, "a short file does not move");
}
