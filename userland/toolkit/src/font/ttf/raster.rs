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

/*
 * The most pixels one glyph box may hold for the glyph to be rasterised.
 * The outline rasteriser allocates a float for every pixel of the box before
 * it draws, so a face scaled far past any screen (a hostile web font whose
 * outlines span thousands of em units, or an absurd font-size) would ask for
 * gigabytes. 1024 x 1024 is 4 MiB of accumulator; a larger glyph draws
 * nothing, like a glyph the face does not have.
 */
pub const MAX_GLYPH_AREA: u64 = 1024 * 1024;

/// A glyph's coverage bitmap (one byte per pixel, row-major) and the offset
/// of its top-left pixel from the pen origin on the baseline.
pub(super) struct Raster {
    pub min_x: i32,
    pub min_y: i32,
    pub w: u32,
    pub h: u32,
    pub cov: Vec<u8>,
}

impl Raster {
    pub fn area(&self) -> u64 {
        self.w as u64 * self.h as u64
    }
}

/* Pixels in a w x h glyph box given as float extents. Saturates, so an
infinite or overflowing extent counts as too large rather than wrapping. */
pub(super) fn box_area(w: f32, h: f32) -> u64 {
    (w as u64).saturating_mul(h as u64)
}

/* Largest whole number not above `x`, without libm. Saturates to the i32
range, far beyond any pen or ascent a surface can show. */
pub(super) fn floor(x: f32) -> f32 {
    let t = x as i32 as f32;
    if t > x {
        t - 1.0
    } else {
        t
    }
}
