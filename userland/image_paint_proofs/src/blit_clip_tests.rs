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

//! A box starting off screen draws its visible part in place, and a huge
//! box costs only what shows.
use std::time::Instant;
use std::vec::Vec;

use crate::blit_img::{frame, image, FH, FW};
use crate::blit_ref;
use crate::browser::css::ObjectFit;
use crate::browser::image::{blit_into, blit_rect, Decoded};

#[test]
fn a_box_starting_off_screen_draws_its_visible_part_in_place() {
    /* Opaque, so each pixel is the sample alone, whatever it lands on. */
    let mut img = image(16, 16);
    img.px.iter_mut().for_each(|p| *p |= 0xff00_0000);
    let whole = frame(|fb| blit_ref::draw(fb, &img, [0, 0, 64, 60], [0, 0, 16, 16], 255, None));
    let shifted = frame(|fb| blit_rect(fb, &img, [-20, -30, 64, 60], ObjectFit::FILL, 255, None));
    let base = frame(|_| {});
    for y in 0..FH as usize {
        for x in 0..FW as usize {
            let want = if x < 44 && y < 30 {
                whole[(y + 30) * FW as usize + x + 20]
            } else {
                base[y * FW as usize + x]
            };
            assert_eq!(shifted[y * FW as usize + x], want, "pixel {x},{y}");
        }
    }
}

#[test]
fn a_20000_pixel_box_costs_what_shows() {
    let img = image(64, 48);
    let t = Instant::now();
    for fit in [ObjectFit::FILL, ObjectFit::COVER, ObjectFit::CONTAIN] {
        frame(|fb| {
            blit_rect(fb, &img, [-9000, -9000, 20000, 20000], fit, 255, Some([0, 0, 97, 61]))
        });
    }
    assert!(t.elapsed().as_millis() < 50, "took {:?}", t.elapsed());
    let empty = Decoded { w: 0, h: 0, px: Vec::new() };
    frame(|fb| blit_into(fb, &empty, [0, 0, 10, 10], ObjectFit::COVER, 255, None));
    frame(|fb| blit_into(fb, &img, [0, 0, 0, 10], ObjectFit::CONTAIN, 255, None));
}
