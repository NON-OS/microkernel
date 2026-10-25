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

//! The canvas doubled onto a dense screen: every screen pixel is the canvas
//! pixel it covers, nothing outside the rectangle asked for is written, and
//! a screen an odd pixel wider than twice the canvas loses nothing to it.

use super::upscale_buf::*;
use crate::damage::Rect;
use crate::sw_blitter::upscale;

#[test]
fn each_canvas_pixel_becomes_a_square_on_the_screen() {
    let mut canvas = Buf::new(4, 3, distinct);
    let mut screen = Buf::new(8, 6, |_, _| 0);
    let all = Rect { x: 0, y: 0, width: 4, height: 3 };
    let out = upscale(canvas.surface(), screen.surface(), all, 2);
    assert_eq!(t(out), Some((0, 0, 8, 6)));
    for y in 0..6 {
        for x in 0..8 {
            assert_eq!(screen.at(x, y), canvas.at(x / 2, y / 2), "screen ({x}, {y})");
        }
    }
    assert!(screen.guards_intact());
}

#[test]
fn only_the_rectangle_asked_for_is_written() {
    let mut canvas = Buf::new(4, 3, distinct);
    let mut screen = Buf::new(8, 6, |_, _| 7);
    let out =
        upscale(canvas.surface(), screen.surface(), Rect { x: 1, y: 1, width: 2, height: 1 }, 2);
    assert_eq!(t(out), Some((2, 2, 4, 2)));
    for y in 0..6 {
        for x in 0..8 {
            let inside = (2..6).contains(&x) && (2..4).contains(&y);
            let want = if inside { canvas.at(x / 2, y / 2) } else { 7 };
            assert_eq!(screen.at(x, y), want, "screen ({x}, {y})");
        }
    }
    assert!(screen.guards_intact());
}

#[test]
fn an_odd_screen_keeps_its_last_column_and_row_and_writes_nothing_past_them() {
    /* 9 by 7 halves to a canvas of 4 by 3; the ninth column and seventh row
     * are the screen's own and are never written. */
    let mut canvas = Buf::new(4, 3, distinct);
    let mut screen = Buf::new(9, 7, |_, _| 7);
    let out =
        upscale(canvas.surface(), screen.surface(), Rect { x: 0, y: 0, width: 4, height: 3 }, 2);
    assert_eq!(t(out), Some((0, 0, 8, 6)));
    assert!((0..7).all(|y| screen.at(8, y) == 7));
    assert!((0..9).all(|x| screen.at(x, 6) == 7));
    assert!(screen.guards_intact());
}
