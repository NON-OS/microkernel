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

use crate::damage::Rect;
use crate::sw_blitter::{upscale, Surface};

const GUARD: usize = 16;

/// A buffer of `w` by `h` with guard cells either side, so any write that
/// leaves the surface lands where it is caught.
struct Buf {
    w: u32,
    h: u32,
    px: Vec<u32>,
}

impl Buf {
    fn new(w: u32, h: u32, fill: impl Fn(u32, u32) -> u32) -> Buf {
        let mut px = vec![0u32; GUARD + (w * h) as usize + GUARD];
        for y in 0..h {
            for x in 0..w {
                px[GUARD + (y * w + x) as usize] = fill(x, y);
            }
        }
        Buf { w, h, px }
    }
    fn surface(&mut self) -> Surface {
        let base = unsafe { self.px.as_mut_ptr().add(GUARD) } as u64;
        let (w, h) = (self.w, self.h);
        Surface { base_va: base, stride: w * 4, width: w, height: h, byte_len: (w * h * 4) as u64 }
    }
    fn at(&self, x: u32, y: u32) -> u32 {
        self.px[GUARD + (y * self.w + x) as usize]
    }
    fn guards_intact(&self) -> bool {
        let n = self.px.len();
        self.px[..GUARD].iter().chain(&self.px[n - GUARD..]).all(|&p| p == 0)
    }
}

/// A rectangle as a tuple, which the in-tree `Rect` does not need to be.
fn t(r: Option<Rect>) -> Option<(u32, u32, u32, u32)> {
    r.map(|r| (r.x, r.y, r.width, r.height))
}

fn distinct(x: u32, y: u32) -> u32 {
    0xFF00_0000 | (y << 8) | x
}

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
    let out = upscale(canvas.surface(), screen.surface(), Rect { x: 1, y: 1, width: 2, height: 1 }, 2);
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
    let out = upscale(canvas.surface(), screen.surface(), Rect { x: 0, y: 0, width: 4, height: 3 }, 2);
    assert_eq!(t(out), Some((0, 0, 8, 6)));
    assert!((0..7).all(|y| screen.at(8, y) == 7));
    assert!((0..9).all(|x| screen.at(x, 6) == 7));
    assert!(screen.guards_intact());
}

#[test]
fn damage_hanging_off_the_canvas_is_clipped_and_damage_beyond_it_is_nothing() {
    let mut canvas = Buf::new(4, 3, distinct);
    let mut screen = Buf::new(8, 6, |_, _| 0);
    let hanging = Rect { x: 3, y: 2, width: 50, height: 50 };
    let out = upscale(canvas.surface(), screen.surface(), hanging, 2);
    assert_eq!(t(out), Some((6, 4, 2, 2)));
    assert_eq!(screen.at(7, 5), canvas.at(3, 2));
    let beyond = Rect { x: 4, y: 0, width: 1, height: 1 };
    assert_eq!(t(upscale(canvas.surface(), screen.surface(), beyond, 2)), None);
    let empty = Rect { x: 0, y: 0, width: 0, height: 1 };
    assert_eq!(t(upscale(canvas.surface(), screen.surface(), empty, 2)), None);
    assert!(screen.guards_intact());
}

#[test]
fn a_scale_of_one_is_a_plain_copy_and_zero_is_refused() {
    let mut canvas = Buf::new(4, 3, distinct);
    let mut screen = Buf::new(4, 3, |_, _| 0);
    let all = Rect { x: 0, y: 0, width: 4, height: 3 };
    assert_eq!(t(upscale(canvas.surface(), screen.surface(), all, 1)), Some((0, 0, 4, 3)));
    assert!((0..3).all(|y| (0..4).all(|x| screen.at(x, y) == canvas.at(x, y))));
    assert_eq!(t(upscale(canvas.surface(), screen.surface(), all, 0)), None);
}
