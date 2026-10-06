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

//! The wheel rule every scrolled view shares (app_skeleton's scroll): a
//! notch away from the user (+1, as the PS/2, USB and I2C drivers and the
//! touchpad's two-finger scroll all post it) moves toward the top, a step per
//! notch, and the view never leaves `0..=max`.

use nonos_app_skeleton::scroll::{wheel_offset, wheel_px, MAX_NOTCHES, WHEEL_LINES};

#[test]
fn a_notch_away_moves_toward_the_top_and_a_notch_toward_moves_down() {
    assert_eq!(wheel_offset(10, 1, WHEEL_LINES, 100), 7);
    assert_eq!(wheel_offset(10, -1, WHEEL_LINES, 100), 13);
    assert_eq!(wheel_offset(10, 2, WHEEL_LINES, 100), 4, "two notches, two steps");
}

#[test]
fn neither_end_is_passed() {
    assert_eq!(wheel_offset(2, 1, WHEEL_LINES, 100), 0, "the top holds");
    assert_eq!(wheel_offset(0, 1, WHEEL_LINES, 100), 0);
    assert_eq!(wheel_offset(98, -1, WHEEL_LINES, 100), 100, "the end holds");
    assert_eq!(wheel_offset(100, -5, WHEEL_LINES, 100), 100);
    assert_eq!(wheel_offset(5, -1, WHEEL_LINES, 0), 0, "content that fits never scrolls");
    // An offset left past a list that has since shrunk comes back to its end
    // on the next notch, either way.
    assert_eq!(wheel_offset(50, 1, WHEEL_LINES, 20), 20);
}

#[test]
fn a_flick_is_capped_and_extreme_deltas_do_not_overflow() {
    let cap = MAX_NOTCHES as usize * WHEEL_LINES;
    assert_eq!(wheel_offset(0, -1000, WHEEL_LINES, usize::MAX), cap);
    assert_eq!(wheel_offset(0, i32::MIN, WHEEL_LINES, usize::MAX), cap);
    assert_eq!(wheel_offset(usize::MAX, i32::MAX, usize::MAX, usize::MAX), 0);
    assert_eq!(wheel_offset(usize::MAX - 1, i32::MIN, usize::MAX, usize::MAX), usize::MAX);
}

#[test]
fn pixel_views_take_the_same_rule() {
    assert_eq!(wheel_px(100, 1, 48, 500), 52);
    assert_eq!(wheel_px(480, -1, 48, 500), 500);
    assert_eq!(wheel_px(30, 1, 48, 500), 0);
}

#[test]
fn no_vertical_notch_moves_nothing_but_an_offset_past_the_end() {
    assert_eq!(wheel_offset(6, 0, WHEEL_LINES, 9), 6);
    assert_eq!(wheel_offset(12, 0, WHEEL_LINES, 9), 9);
}
