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

/* An interlaced frame stores its rows in four passes: every 8th row from 0,
 * every 8th from 4, every 4th from 2, every 2nd from 1. */
const PASSES: [(usize, usize); 4] = [(0, 8), (4, 8), (2, 4), (1, 2)];

/* One frame's geometry within the logical screen. */
pub(super) struct Frame {
    pub w: usize,
    pub h: usize,
    pub left: usize,
    pub top: usize,
    pub interlace: bool,
    /* Stream row at which each pass begins, and the row count as the end. */
    first: [usize; 5],
}

impl Frame {
    pub fn new(w: usize, h: usize, left: usize, top: usize, interlace: bool) -> Self {
        let mut first = [0usize; 5];
        for (p, &(start, step)) in PASSES.iter().enumerate() {
            first[p + 1] = first[p] + h.saturating_sub(start).div_ceil(step);
        }
        Self { w, h, left, top, interlace, first }
    }

    /* Frame row of stream row `r`, in constant time; None past the end. */
    pub fn row(&self, r: usize) -> Option<usize> {
        if r >= self.h {
            return None;
        }
        if !self.interlace {
            return Some(r);
        }
        let p = (0..4).find(|&p| r < self.first[p + 1])?;
        Some(PASSES[p].0 + (r - self.first[p]) * PASSES[p].1)
    }

    /* Stream row that carries frame row `y` (the inverse of `row`). */
    fn stream_row(&self, y: usize) -> usize {
        if !self.interlace {
            return y;
        }
        let p = match (y % 8, y % 2) {
            (_, 1) => 3,
            (2 | 6, _) => 2,
            (4, _) => 1,
            _ => 0,
        };
        self.first[p] + (y - PASSES[p].0) / PASSES[p].1
    }

    /* Stream rows to decode so frame rows 0..visible are all filled: the
     * latest stream position among them, plus one. */
    pub fn rows_needed(&self, visible: usize) -> usize {
        let last =
            |&(s, st): &(usize, usize)| (visible > s).then(|| s + (visible - 1 - s) / st * st);
        PASSES.iter().filter_map(last).map(|y| self.stream_row(y) + 1).max().unwrap_or(0)
    }
}
