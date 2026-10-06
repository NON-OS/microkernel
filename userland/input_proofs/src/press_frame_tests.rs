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

//! The press grab's frame. A window that drags itself moves while the button
//! is held, so the router delivers motion relative to where the window was at
//! the press: the drag then moves it by exactly what the pointer moved, never
//! by what the pointer moved plus what the window already moved.

use crate::router_press::Press;

#[test]
fn the_frame_is_the_window_origin_at_the_press() {
    let p = Press::arm(7, 260, 125, 160, 25);
    assert_eq!(p.pid, 7);
    assert_eq!((p.origin_x, p.origin_y), (100, 100));
    assert_eq!(p.local(260, 125), (160, 25), "the press point maps back to itself");
}

#[test]
fn motion_arrives_moved_by_exactly_the_screen_delta() {
    let p = Press::arm(7, 260, 125, 160, 25);
    for (dx, dy) in [(1i32, 0i32), (0, 1), (-40, 17), (300, -90), (-160, -25)] {
        let x = (260 + dx) as u32;
        let y = (125 + dy) as u32;
        assert_eq!(p.local(x, y), (160 + dx, 25 + dy));
    }
}

#[test]
fn the_frame_does_not_follow_the_window() {
    // The press arms once; later motion is relative to that origin however
    // far the window has dragged itself since.
    let p = Press::arm(3, 50, 50, 10, 10);
    assert_eq!(p.local(450, 50), (410, 10));
    assert_eq!(p.local(0, 0), (-40, -40), "above and left of the window is negative");
}

#[test]
fn extreme_points_saturate_instead_of_wrapping() {
    let p = Press::arm(1, u32::MAX, u32::MAX, 0, 0);
    let (x, y) = p.local(0, 0);
    assert!(x <= 0 && y <= 0, "no wrap to a far positive coordinate: ({x}, {y})");
}
