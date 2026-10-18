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

//! A guarded canvas for the blitter proofs: a real `Surface` on a heap
//! buffer with guard bytes either side, and an opaque source to copy.

use crate::sw_blitter::Surface;

pub(crate) const W: u32 = 40;
pub(crate) const H: u32 = 30;
pub(crate) const GUARD: usize = 16;

// A destination buffer with guard cells before and after the real surface, so
// any out-of-bounds write lands in a guard and is caught.
pub(crate) struct Canvas {
    pub(crate) buf: Vec<u32>,
}

impl Canvas {
    pub(crate) fn new() -> Self {
        Canvas { buf: vec![0u32; GUARD + (W * H) as usize + GUARD] }
    }
    pub(crate) fn surface(&mut self) -> Surface {
        let base = unsafe { self.buf.as_mut_ptr().add(GUARD) } as u64;
        Surface { base_va: base, stride: W * 4, width: W, height: H, byte_len: (W * H * 4) as u64 }
    }
    pub(crate) fn assert_guards_intact(&self) {
        for i in 0..GUARD {
            assert_eq!(self.buf[i], 0, "front guard {i} clobbered");
            assert_eq!(self.buf[self.buf.len() - 1 - i], 0, "back guard {i} clobbered");
        }
    }
    pub(crate) fn pixel(&self, x: u32, y: u32) -> u32 {
        self.buf[GUARD + (y * W + x) as usize]
    }
}

pub(crate) fn opaque_src(w: u32, h: u32, color: u32) -> (Vec<u32>, Surface) {
    let mut pixels = vec![color; (w * h) as usize];
    let base = pixels.as_mut_ptr() as u64;
    let s =
        Surface { base_va: base, stride: w * 4, width: w, height: h, byte_len: (w * h * 4) as u64 };
    (pixels, s)
}
