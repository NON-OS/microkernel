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

//! Every app's window keeps at least three quarters of the size it opened at.
//! Process Manager dragged far below its 1240 by 780 drew its panes over each
//! other; now no window goes below the size its layout holds at.

use crate::runner::min_size::{floor, settle, MIN_H, MIN_W};

#[test]
fn the_floor_is_three_quarters_of_the_opening_size() {
    assert_eq!(floor(1240, 780), (930, 585));
    assert_eq!(floor(960, 540), (720, 405));
}

#[test]
fn a_small_window_keeps_the_drag_minimum() {
    assert_eq!(floor(360, 180), (MIN_W, MIN_H));
}

#[test]
fn a_drag_below_the_floor_stops_at_it() {
    assert_eq!(settle((400, 300), (1240, 780), Some((1920, 1080))), (930, 585));
}

#[test]
fn a_drag_above_the_floor_is_taken_as_asked() {
    assert_eq!(settle((1000, 700), (1240, 780), Some((1920, 1080))), (1000, 700));
}

#[test]
fn the_display_bounds_the_window_whatever_the_floor() {
    assert_eq!(settle((400, 300), (1680, 1000), Some((1024, 700))), (1024, 700));
    assert_eq!(settle((3000, 3000), (960, 540), Some((1920, 1080))), (1920, 1080));
}

#[test]
fn no_display_answer_leaves_only_the_floor() {
    assert_eq!(settle((10, 10), (960, 540), None), (720, 405));
}
