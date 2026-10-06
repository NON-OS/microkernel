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

//! Moving the selection keeps it on the page that is drawn: the list scrolls
//! by rows, the grid by whole lines of tiles, and only once the selection
//! would leave the screen. Every page but the player draws the side rail.

use crate::ui::rows::{scroll_for, wheel_scroll};
use crate::ui::screen::Route;

#[test]
fn a_wheel_notch_moves_the_grid_a_line_of_tiles() {
    // Four columns, three lines on screen, 30 videos: eight lines in all.
    assert_eq!(wheel_scroll(0, 30, 12, 4, -1), 4, "a notch toward the user: one line on");
    assert_eq!(wheel_scroll(8, 30, 12, 4, 1), 4, "a notch away: one line back");
    assert_eq!(wheel_scroll(0, 30, 12, 4, -2), 8);
    // A start left mid-line snaps to its line as it moves.
    assert_eq!(wheel_scroll(5, 30, 12, 4, -1), 8);
}

#[test]
fn a_wheel_notch_moves_the_list_three_rows() {
    assert_eq!(wheel_scroll(0, 40, 8, 1, -1), 3);
    assert_eq!(wheel_scroll(10, 40, 8, 1, 1), 7);
}

#[test]
fn the_wheel_stops_at_the_top_and_with_the_last_line_on_the_page() {
    // Eight lines, three shown: the fifth line is as far as the page starts.
    assert_eq!(wheel_scroll(20, 30, 12, 4, -1), 20);
    assert_eq!(wheel_scroll(16, 30, 12, 4, -5), 20);
    assert_eq!(wheel_scroll(4, 30, 12, 4, 3), 0);
    assert_eq!(wheel_scroll(38, 40, 8, 1, -1), 32, "a list left past its end comes back");
    assert_eq!(wheel_scroll(0, 10, 12, 4, -1), 0, "a library that fits does not move");
    assert_eq!(wheel_scroll(0, 0, 0, 0, -1), 0);
}

#[test]
fn the_list_scrolls_only_when_the_selection_leaves_it() {
    // Eight rows fit: moving within them keeps the first row where it is.
    assert_eq!(scroll_for(7, 0, 8, 1), 0);
    // One past the last drawn row brings it in as the last row.
    assert_eq!(scroll_for(8, 0, 8, 1), 1);
    assert_eq!(scroll_for(20, 0, 8, 1), 13);
    // Moving up above the first drawn row makes it the first.
    assert_eq!(scroll_for(4, 10, 8, 1), 4);
    assert_eq!(scroll_for(12, 10, 8, 1), 10);
}

#[test]
fn the_grid_scrolls_by_whole_lines() {
    // Four columns, three lines on screen: twelve tiles.
    assert_eq!(scroll_for(11, 0, 12, 4), 0);
    // The first tile of the fourth line scrolls one line, never one tile.
    assert_eq!(scroll_for(12, 0, 12, 4), 4);
    assert_eq!(scroll_for(15, 0, 12, 4), 4);
    // Back up into the first line from the second.
    assert_eq!(scroll_for(2, 4, 12, 4), 0);
    // A start left mid-line (the view was just switched) snaps to its line.
    assert_eq!(scroll_for(6, 5, 12, 4), 4);
}

#[test]
fn a_page_with_no_room_still_shows_the_selection() {
    assert_eq!(scroll_for(5, 0, 0, 1), 5);
    assert_eq!(scroll_for(5, 0, 0, 0), 5);
    assert_eq!(scroll_for(9, 0, 2, 4), 8);
}

#[test]
fn only_the_player_goes_without_the_rail() {
    assert!(Route::Library.chrome());
    assert!(Route::Files.chrome());
    assert!(Route::Details.chrome());
    assert!(!Route::Player.chrome());
}
