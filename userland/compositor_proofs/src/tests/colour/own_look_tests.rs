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

//! An output's look laid over its own copy of the true-colour canvas leaves
//! the indicator band's pixels true and changes every other one.

use super::own_colour_tests::GREY;
use crate::damage::Rect;
use crate::frame_pacer::indicator_band::{band, BAND_ARGB, CAMERA_ON};
use crate::frame_pacer::look::build;
use crate::frame_pacer::own_look::lay;
use crate::sw_blitter::Surface;

#[test]
fn a_look_laid_over_a_copy_keeps_the_band_true() {
    let (w, h) = (200u32, 40u32);
    let mut px: Vec<u32> = (0..w * h).map(|_| 0xFFFF_2010).collect();
    let b = band(w, h);
    for y in b.y..b.y + b.height {
        for x in b.x..b.x + b.width {
            px[(y * w + x) as usize] = if x == b.x + 8 { CAMERA_ON } else { BAND_ARGB };
        }
    }
    let before = px.clone();
    let byte_len = px.len() as u64 * 4;
    let s =
        Surface { base_va: px.as_mut_ptr() as u64, stride: w * 4, width: w, height: h, byte_len };
    let look = build(&GREY, None).unwrap();
    lay(s, Rect { x: 0, y: 0, width: w, height: h }, (w, h), &look);
    for y in 0..h {
        for x in 0..w {
            let i = (y * w + x) as usize;
            let inside = x >= b.x && y < b.y + b.height;
            assert_eq!(px[i] == before[i], inside, "({x}, {y})");
        }
    }
}
