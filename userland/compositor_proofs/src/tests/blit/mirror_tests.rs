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

//! The screen drawn onto a mirrored output of another size: fitted whole,
//! shape kept and centred, nearest pixel, nothing outside the damage drawn.

use crate::state::damage::Rect;
use crate::sw_blitter::resample::{fit_box, resample};
use crate::sw_blitter::Surface;

struct Buf {
    px: Vec<u32>,
    w: u32,
    h: u32,
}

impl Buf {
    fn new(w: u32, h: u32, f: impl Fn(u32, u32) -> u32) -> Buf {
        let px = (0..w * h).map(|i| f(i % w, i / w)).collect();
        Buf { px, w, h }
    }
    fn surface(&mut self) -> Surface {
        let (stride, byte_len) = (self.w * 4, self.px.len() as u64 * 4);
        Surface {
            base_va: self.px.as_mut_ptr() as u64,
            stride,
            width: self.w,
            height: self.h,
            byte_len,
        }
    }
    fn at(&self, x: u32, y: u32) -> u32 {
        self.px[(y * self.w + x) as usize]
    }
}

#[test]
fn a_wider_output_gets_bars_left_and_right() {
    assert_eq!(fit_box((1920, 1080), (1920, 1200)), (0, 60, 1920, 1080), "16:10 below 16:9");
    assert_eq!(fit_box((1280, 1024), (1920, 1080)), (285, 0, 1350, 1080), "5:4 on 16:9");
    assert_eq!(fit_box((1920, 1080), (960, 540)), (0, 0, 960, 540), "the same shape halved");
}

#[test]
fn a_halved_output_takes_every_other_pixel_inside_the_box() {
    let mut src = Buf::new(8, 4, |x, y| y * 100 + x);
    let mut dst = Buf::new(4, 2, |_, _| 7);
    let out = resample(src.surface(), dst.surface(), Rect { x: 0, y: 0, width: 8, height: 4 });
    assert_eq!(out.map(|r| (r.x, r.y, r.width, r.height)), Some((0, 0, 4, 2)));
    assert_eq!((dst.at(0, 0), dst.at(3, 1)), (src.at(1, 1), src.at(7, 3)));
}

#[test]
fn only_the_damaged_part_and_never_the_bars_are_written() {
    let mut src = Buf::new(4, 4, |x, y| 1 + y * 4 + x);
    let mut dst = Buf::new(8, 4, |_, _| 0);
    let out = resample(src.surface(), dst.surface(), Rect { x: 0, y: 0, width: 1, height: 1 });
    assert_eq!(out.map(|r| (r.x, r.y, r.width, r.height)), Some((2, 0, 1, 1)), "centred");
    assert_eq!(dst.at(2, 0), src.at(0, 0));
    assert_eq!(dst.px.iter().filter(|&&p| p != 0).count(), 1);
    assert!((0..4).all(|y| dst.at(0, y) == 0 && dst.at(7, y) == 0), "bars untouched");
}
