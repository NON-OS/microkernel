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

use super::blend::mix;
use super::raster::{Raster, MAX_GLYPH_AREA};

/* The ARGB8888 surface text draws into: `w` x `h` visible pixels, rows
`stride` words apart. */
pub(super) struct Target<'a> {
    pub buf: &'a mut [u32],
    pub stride: usize,
    pub w: u32,
    pub h: u32,
}

impl Target<'_> {
    /* The largest glyph box drawn here: the fixed cap, or the surface's own
    area when that is smaller. */
    pub fn glyph_limit(&self) -> u64 {
        MAX_GLYPH_AREA.min(self.w as u64 * self.h as u64)
    }

    /* Blend one coverage sample at (x, y); off-surface samples are dropped. */
    pub fn blend(&mut self, x: i32, y: i32, argb: u32, cov: u8) {
        if cov == 0 || x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return;
        }
        if let Some(p) = self.buf.get_mut(y as usize * self.stride + x as usize) {
            *p = mix(*p, argb, cov);
        }
    }

    /* Blend a raster whose top-left pixel lands at (ox, oy). The box is
    clipped to the surface once, then each row is a slice walk. */
    pub fn blit(&mut self, r: &Raster, ox: i32, oy: i32, argb: u32) {
        let (x0, x1) = span(ox, r.w, self.w);
        let (y0, y1) = span(oy, r.h, self.h);
        if x0 == x1 {
            return;
        }
        for y in y0..y1 {
            let row = y as usize * self.stride;
            let end = (row + x1 as usize).min(self.buf.len());
            let Some(dst) = self.buf.get_mut(row + x0 as usize..end) else { break };
            let src = (y - oy as i64) as usize * r.w as usize + (x0 - ox as i64) as usize;
            let Some(cov) = r.cov.get(src..) else { break };
            for (d, &c) in dst.iter_mut().zip(cov) {
                if c != 0 {
                    *d = mix(*d, argb, c);
                }
            }
        }
    }
}

/* The part of [at, at + len) inside [0, limit), empty when none is. */
fn span(at: i32, len: u32, limit: u32) -> (i64, i64) {
    let lo = (at as i64).max(0);
    let hi = (at as i64 + len as i64).min(limit as i64);
    (lo, hi.max(lo))
}
