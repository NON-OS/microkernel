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

use alloc::vec::Vec;

/* Working canvas at 2x the output size; the box filter on the way out is
 * the antialiasing. */
pub(super) const SS: u32 = 2;

/// One band of the supersampled canvas: `h` rows starting at device row
/// `y0`, so a large raster is drawn a band at a time in bounded memory.
pub(super) struct Raster {
    pub w: u32,
    pub h: u32,
    pub y0: i32,
    pub px: Vec<u32>,
}

impl Raster {
    /// The band of `rows` output rows from output row `from`, or None when
    /// its memory cannot be had.
    pub fn band(out_w: u32, rows: u32, from: u32) -> Option<Self> {
        let (w, h) = (out_w * SS, rows * SS);
        let mut px = Vec::new();
        px.try_reserve_exact(w as usize * h as usize).ok()?;
        px.resize(w as usize * h as usize, 0);
        Some(Raster { w, h, y0: (from * SS) as i32, px })
    }

    /// An empty band the size and place of `other`, for a clip mask.
    pub fn like(other: &Raster) -> Option<Self> {
        let r = Raster::band(other.w / SS, other.h / SS, 0)?;
        Some(Raster { y0: other.y0, ..r })
    }

    /// The device rows this band holds, end exclusive.
    pub fn rows(&self) -> (i32, i32) {
        (self.y0, self.y0 + self.h as i32)
    }

    pub(super) fn index(&self, x: i32, y: i32) -> Option<usize> {
        let y = y - self.y0;
        let inside = x >= 0 && y >= 0 && x < self.w as i32 && y < self.h as i32;
        inside.then(|| y as usize * self.w as usize + x as usize)
    }

    /// Coverage alpha at device pixel (x, y), zero outside the band.
    pub fn alpha(&self, x: i32, y: i32) -> u32 {
        self.index(x, y).map_or(0, |i| self.px[i] >> 24)
    }
}
