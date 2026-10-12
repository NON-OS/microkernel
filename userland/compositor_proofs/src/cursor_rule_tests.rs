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

//! Where a canvas point lands on the screen and on a mirrored output for a
//! device's cursor plane, and the CURSOR each becomes.

use nonos_gfx_wire::body::cursor::{NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::Cursor;

use crate::display::cursor_rule::{message, on_head, on_screen};

#[test]
fn a_canvas_point_is_scaled_or_shrunk_onto_the_screen() {
    assert_eq!(on_screen((100, 50), (1920, 1080), (3840, 2160), 8), (200, 100));
    assert_eq!(on_screen((100, 50), (2560, 1440), (3840, 2160), 6), (150, 75), "1.5 times");
    assert_eq!(on_screen((100, 50), (1920, 1080), (1920, 1080), 4), (100, 50));
    let floor = on_screen((960, 540), (1920, 1080), (1280, 720), 4);
    assert_eq!(floor, (640, 360), "the floor canvas, shrunk");
}

#[test]
fn a_mirrored_output_gets_the_point_inside_its_fitted_box() {
    // 1920x1080 fitted in 1024x768: 1024x576, 96 rows down.
    assert_eq!(on_head((960, 540), (1920, 1080), (1024, 768)), (512, 384));
    assert_eq!(on_head((0, 0), (1920, 1080), (1024, 768)), (0, 96));
    assert_eq!(on_head((-5, -5), (1920, 1080), (1024, 768)), (0, 96), "never above the box");
    assert_eq!(on_head((7, 9), (1920, 1080), (1920, 1080)), (7, 9));
}

#[test]
fn cursor_says_shown_and_new_shape_only_when_they_are() {
    let c = message((10, 20), true, true, (11, 19));
    let want = Cursor {
        flags: VISIBLE | NEW_SHAPE,
        x: 10,
        y: 20,
        hot_x: 0,
        hot_y: 0,
        width: 11,
        height: 19,
    };
    assert_eq!(c, want);
    assert_eq!(message((10, 20), true, false, (11, 19)).flags, VISIBLE);
    assert_eq!(message((0, 0), false, false, (0, 0)).flags, 0, "hidden");
    assert_eq!(
        Cursor::decode(&message((0, 0), false, false, (0, 0)).encode()).map(|c| c.flags),
        Ok(0)
    );
}
