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

//! From the kernel's two answers to the logo shown: pixel for pixel at the
//! firmware's own mode, turned as the firmware turned it, halved on a doubled
//! canvas, and placed from its margins when the mode is unknown.

use crate::build::{bmp, colour, record};
use crate::logo::place::Spot;
use crate::logo::prepare::prepare;
use crate::logo::record::{ON_SCREEN, TOP_DOWN};

#[test]
fn at_the_firmware_mode_the_logo_comes_back_pixel_for_pixel() {
    for (bpp, top_down) in [(24, false), (32, true)] {
        let f = bmp(30, 20, bpp, top_down);
        let flags = ON_SCREEN | if top_down { TOP_DOWN } else { 0 };
        let s = prepare(&record(5, 7, &f, flags, 0), &f, Some((40, 34)), (40, 34)).unwrap();
        assert_eq!(s.spot, Spot { x: 5, y: 7, w: 30, h: 20 });
        let want: Vec<u32> = (0..20).flat_map(|y| (0..30).map(move |x| colour(x, y))).collect();
        assert_eq!(s.pixels, want);
    }
}

#[test]
fn a_turned_logo_is_shown_turned() {
    let f = bmp(3, 2, 24, false);
    let s = prepare(&record(0, 0, &f, ON_SCREEN, 1), &f, Some((2, 3)), (2, 3)).unwrap();
    assert_eq!((s.spot.w, s.spot.h), (2, 3));
    assert_eq!(&s.pixels[..2], &[colour(0, 1), colour(0, 0)], "left column on top");
}

#[test]
fn a_doubled_canvas_shows_every_other_pixel_and_no_mode_means_the_margins() {
    let f = bmp(8, 4, 32, false);
    let s = prepare(&record(4, 2, &f, ON_SCREEN, 0), &f, Some((16, 8)), (8, 4)).unwrap();
    assert_eq!(s.spot, Spot { x: 2, y: 1, w: 4, h: 2 });
    assert_eq!(s.pixels[..4], [colour(1, 1), colour(3, 1), colour(5, 1), colour(7, 1)]);
    let guessed = prepare(&record(4, 2, &f, ON_SCREEN, 0), &f, None, (8, 4)).unwrap();
    assert_eq!(guessed.spot, s.spot, "centred: the margins give the mode");
}
