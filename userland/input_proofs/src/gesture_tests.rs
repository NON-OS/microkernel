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

//! The touchpad gesture state machine's motion: relative, accelerated, slow
//! motion carried between reports, every report capped, and a torn
//! coordinate re-anchored instead of followed.

use crate::gesture::TouchGesture;
use crate::hid::TouchSample;

/// One report from a 1000 by 1000 pad with one confident finger down.
pub(super) fn at(x: u32, y: u32, tip: bool) -> TouchSample {
    let contacts = u32::from(tip);
    TouchSample {
        x,
        y,
        x_max: 1000,
        y_max: 1000,
        tip,
        contacts,
        contact_id: None,
        button: false,
        confidence: true,
    }
}

#[test]
fn the_first_report_of_a_touch_moves_nothing_and_the_next_moves_by_the_delta() {
    let mut g = TouchGesture::default();
    assert_eq!(g.on_touch(&at(500, 250, true)).motion, None);
    /* 10 units: speed 8, gain 10/16, 10 * 800 * 10 / 16000 = 5 pixels. */
    let a = g.on_touch(&at(510, 250, true));
    assert_eq!(a.motion, Some((5, 0)));
    assert!(!a.button_down && !a.button_up && a.wheel == 0);
}

#[test]
fn motion_below_a_pixel_adds_up_across_reports() {
    let mut g = TouchGesture::default();
    g.on_touch(&at(500, 500, true));
    /* One unit at the floor gain is 6400 / 16000 of a pixel: two carry, the third moves. */
    assert_eq!(g.on_touch(&at(501, 500, true)).motion, None);
    assert_eq!(g.on_touch(&at(502, 500, true)).motion, None);
    assert_eq!(g.on_touch(&at(503, 500, true)).motion, Some((1, 0)));
}

#[test]
fn no_report_moves_the_cursor_past_the_cap() {
    let mut g = TouchGesture::default();
    g.on_touch(&at(100, 900, true));
    /* 125 units, the largest continuous step: 137 pixels uncapped. */
    assert_eq!(g.on_touch(&at(225, 775, true)).motion, Some((36, -36)));
}

#[test]
fn a_torn_coordinate_re_anchors_without_moving() {
    let mut g = TouchGesture::default();
    g.on_touch(&at(100, 100, true));
    assert_eq!(g.on_touch(&at(400, 100, true)).motion, None);
    assert_eq!(g.on_touch(&at(410, 100, true)).motion, Some((5, 0)));
}
