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

//! The touchpad gesture state machine's clicks and scroll: a short tap is a
//! click and a drag is not, two fingers scroll and never move the cursor, the
//! clickpad button is debounced, and a palm moves nothing until it lifts.

use super::gesture_tests::at;
use crate::gesture::TouchGesture;

#[test]
fn a_tap_in_place_is_one_click_and_a_drag_is_none() {
    let mut g = TouchGesture::default();
    g.on_touch(&at(500, 250, true));
    let a = g.on_touch(&at(500, 250, false));
    assert!(a.button_down && a.button_up && a.motion.is_none());
    let mut g = TouchGesture::default();
    for x in [100, 200, 300] {
        g.on_touch(&at(x, 100, true));
    }
    let a = g.on_touch(&at(300, 100, false));
    assert!(!a.button_down && !a.button_up);
}

#[test]
fn two_fingers_scroll_by_the_wheel_and_never_move_the_cursor() {
    let mut g = TouchGesture::default();
    let two = |y| crate::hid::TouchSample { contacts: 2, ..at(500, y, true) };
    assert_eq!(g.on_touch(&two(500)).wheel, 0);
    /* 100 units over a step of 1000 / 32 = 31 is 3 notches; fingers moving
     * down the pad scroll the view down, a negative wheel, as traditional
     * scrolling has it. */
    let a = g.on_touch(&two(600));
    assert_eq!((a.wheel, a.motion), (-3, None));
}

#[test]
fn the_clickpad_button_needs_two_agreeing_reports() {
    let mut g = TouchGesture::default();
    let press = |b| crate::hid::TouchSample { button: b, ..at(500, 250, true) };
    assert!(!g.on_touch(&press(true)).button_down);
    assert!(!g.on_touch(&press(false)).button_up);
    assert!(!g.on_touch(&press(true)).button_down);
    assert!(g.on_touch(&press(true)).button_down);
    g.on_touch(&press(false));
    assert!(g.on_touch(&press(false)).button_up);
}

#[test]
fn a_palm_moves_nothing_until_every_contact_lifts() {
    let mut g = TouchGesture::default();
    let palm = |x| crate::hid::TouchSample { confidence: false, ..at(x, 500, true) };
    g.on_touch(&palm(500));
    assert_eq!(g.on_touch(&at(510, 500, true)).motion, None);
    g.on_touch(&at(510, 500, false));
    g.on_touch(&at(510, 500, true));
    assert_eq!(g.on_touch(&at(520, 500, true)).motion, Some((5, 0)));
}
