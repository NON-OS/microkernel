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

//! A rounded rectangle's outline, `t` pixels thick. A row through the
//! corners is coverage-mixed only where a corner's curve is: between the
//! outer and the inner curves the row is the whole colour or nothing, so
//! that run is drawn as one span, and so is the run on either side where
//! the outline is whole. Only the pixels a curve crosses take a square
//! root; a window's shadow is a ring of these a frame edge across, and
//! working every pixel of each corner row cost a resized window most of
//! its frame.

use super::radius;
use super::round_ring::Ring;
use super::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    pub fn stroke_round(&mut self, x: u32, y: u32, w: u32, h: u32, r: u32, t: u32, argb: u32) {
        if t == 0 || w == 0 || h == 0 {
            return;
        }
        if w <= t * 2 || h <= t * 2 {
            self.fill_round(x, y, w, h, r, argb);
            return;
        }
        let r = radius::clamp_radius(w, h, r);
        let iw = w - t * 2;
        let ih = h - t * 2;
        let ir = radius::clamp_radius(iw, ih, r.saturating_sub(t));
        let ring = Ring { x, y, w, h, r, t, iw, ih, ir, argb };
        for row in 0..h {
            if row >= r && row < h - r && t == 1 {
                // A shadow's rings are one pixel thick: a pixel each side.
                self.blend_px(x, y + row, argb);
                self.blend_px(x + w - 1, y + row, argb);
                continue;
            }
            if row >= r && row < h - r {
                self.blend_rect(x, y + row, t, 1, argb);
                self.blend_rect(x + w - t, y + row, t, 1, argb);
                continue;
            }
            self.corner_row(&ring, row);
        }
    }

    /// One row through the corners: the curves pixel by pixel, the runs
    /// between them at once.
    fn corner_row(&mut self, g: &Ring, row: u32) {
        let (lo, hi) = g.straight();
        if lo >= hi {
            self.ring_px(g, row, 0..g.w);
            return;
        }
        self.left_curve(g, row, lo);
        // Between the curves the outer coverage is whole and the inner is
        // whole on the rows it spans, nothing off them (radius::coverage).
        if !g.hole_row(row) {
            self.blend_rect(g.x + lo, g.y + row, hi - lo, 1, g.argb);
        }
        self.right_curve(g, row, hi);
    }

    /// Columns 0..lo: nothing up to the outer curve, then pixel by pixel
    /// until the outline is whole to `lo` (off the hole) or the hole is
    /// (on it), which then holds, as both coverages only rise there.
    fn left_curve(&mut self, g: &Ring, row: u32, lo: u32) {
        for col in g.first_lit(row, 0, lo)..lo {
            let (outer, inner) = (g.outer(col, row), g.inner(col, row));
            if outer == 255 && inner == 255 {
                return;
            }
            if outer == 255 && !g.hole_row(row) {
                self.blend_rect(g.x + col, g.y + row, lo - col, 1, g.argb);
                return;
            }
            self.mixed(g, (col, row), outer.saturating_sub(inner));
        }
    }

    /// Columns hi..w, the mirror of `left_curve`, walked from the right.
    fn right_curve(&mut self, g: &Ring, row: u32, hi: u32) {
        for col in (hi..g.lit_end(row, hi, g.w)).rev() {
            let (outer, inner) = (g.outer(col, row), g.inner(col, row));
            if outer == 255 && inner == 255 {
                return;
            }
            if outer == 255 && !g.hole_row(row) {
                self.blend_rect(g.x + hi, g.y + row, col + 1 - hi, 1, g.argb);
                return;
            }
            self.mixed(g, (col, row), outer.saturating_sub(inner));
        }
    }

    fn ring_px(&mut self, g: &Ring, row: u32, cols: core::ops::Range<u32>) {
        for col in cols {
            self.mixed(g, (col, row), g.coverage(col, row));
        }
    }

    /// The colour at `cov` of 255 on (col, row) of the ring.
    fn mixed(&mut self, g: &Ring, (col, row): (u32, u32), cov: u32) {
        if cov != 0 {
            let (a, rgb) = ((g.argb >> 24) & 0xFF, g.argb & 0x00FF_FFFF);
            self.blend_px(g.x + col, g.y + row, ((a * cov / 255) << 24) | rgb);
        }
    }
}
