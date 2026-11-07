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

//! A rounded outline's shape: its outer box and radius, and its hole's.
//! Across the left half of a corner row the outer and the hole's coverage
//! only rise, and across the right half they only fall, so the run where
//! the outline is mixed is found by a search and then walked, not read a
//! pixel at a time across the row (round_stroke.rs).

use super::radius;

pub(super) struct Ring {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub r: u32,
    pub t: u32,
    pub iw: u32,
    pub ih: u32,
    pub ir: u32,
    pub argb: u32,
}

impl Ring {
    /// The columns no corner's curve reaches, outer or inner: the outer
    /// curve's span is [r, w - r) and the hole's is [t + ir, t + iw - ir).
    pub fn straight(&self) -> (u32, u32) {
        let lo = self.r.max(self.t + self.ir);
        let hi = (self.w - self.r).min(self.t + self.iw - self.ir);
        (lo, hi)
    }

    /// Whether `row` crosses the hole.
    pub fn hole_row(&self, row: u32) -> bool {
        row >= self.t && row < self.h - self.t
    }

    pub fn outer(&self, col: u32, row: u32) -> u32 {
        radius::coverage(col, row, self.w, self.h, self.r)
    }

    pub fn inner(&self, col: u32, row: u32) -> u32 {
        let (t, w) = (self.t, self.w);
        if !self.hole_row(row) || col < t || col >= w - t {
            return 0;
        }
        radius::coverage(col - t, row - t, self.iw, self.ih, self.ir)
    }

    /// What of the colour (col, row) takes: the outer shape less the hole.
    pub fn coverage(&self, col: u32, row: u32) -> u32 {
        match self.outer(col, row) {
            0 => 0,
            outer => outer.saturating_sub(self.inner(col, row)),
        }
    }

    /// The first column of `from..to` the outer shape reaches on `row`,
    /// where its coverage only rises; `to` when none.
    pub fn first_lit(&self, row: u32, from: u32, to: u32) -> u32 {
        let (mut lo, mut hi) = (from, to);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.outer(mid, row) {
                0 => lo = mid + 1,
                _ => hi = mid,
            }
        }
        lo
    }

    /// The end of the columns of `from..to` the outer shape reaches on
    /// `row`, where its coverage only falls; `from` when none.
    pub fn lit_end(&self, row: u32, from: u32, to: u32) -> u32 {
        let (mut lo, mut hi) = (from, to);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.outer(mid, row) {
                0 => hi = mid,
                _ => lo = mid + 1,
            }
        }
        lo
    }
}
