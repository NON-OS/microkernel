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

//! Guarded buffers for the upscale proofs: a surface with guard cells either
//! side, so any write that leaves it lands where it is caught.

use crate::damage::Rect;
use crate::sw_blitter::Surface;

pub(crate) const GUARD: usize = 16;

/// A buffer of `w` by `h` with guard cells either side, so any write that
/// leaves the surface lands where it is caught.
pub(crate) struct Buf {
    pub(crate) w: u32,
    pub(crate) h: u32,
    pub(crate) px: Vec<u32>,
}

impl Buf {
    pub(crate) fn new(w: u32, h: u32, fill: impl Fn(u32, u32) -> u32) -> Buf {
        let mut px = vec![0u32; GUARD + (w * h) as usize + GUARD];
        for y in 0..h {
            for x in 0..w {
                px[GUARD + (y * w + x) as usize] = fill(x, y);
            }
        }
        Buf { w, h, px }
    }
    pub(crate) fn surface(&mut self) -> Surface {
        let base = unsafe { self.px.as_mut_ptr().add(GUARD) } as u64;
        let (w, h) = (self.w, self.h);
        Surface { base_va: base, stride: w * 4, width: w, height: h, byte_len: (w * h * 4) as u64 }
    }
    pub(crate) fn at(&self, x: u32, y: u32) -> u32 {
        self.px[GUARD + (y * self.w + x) as usize]
    }
    pub(crate) fn guards_intact(&self) -> bool {
        let n = self.px.len();
        self.px[..GUARD].iter().chain(&self.px[n - GUARD..]).all(|&p| p == 0)
    }
}

/// A rectangle as a tuple, which the in-tree `Rect` does not need to be.
pub(crate) fn t(r: Option<Rect>) -> Option<(u32, u32, u32, u32)> {
    r.map(|r| (r.x, r.y, r.width, r.height))
}

pub(crate) fn distinct(x: u32, y: u32) -> u32 {
    0xFF00_0000 | (y << 8) | x
}
