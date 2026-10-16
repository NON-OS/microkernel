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

//! Taking a rectangle in, and coalescing until no two touch.
//!
//! Folding a rectangle into the first one it touched used to stop there, but
//! the grown rectangle could now overlap another one held: the pixels both
//! covered were composed twice a frame, and a run of small damage along a
//! line kept the set full of overlapping pieces. The grown one is taken out
//! and put in again now, so it keeps folding until it touches nothing, and
//! the rectangles drained are always pairwise apart.

use super::accumulator::{DamageAccumulator, MAX_RECTS};
use super::rect::Rect;

impl DamageAccumulator {
    pub fn accumulate(&mut self, r: Rect) {
        if r.is_empty() {
            return;
        }
        let mut r = r;
        // Each pass removes one held rectangle, so this ends within MAX_RECTS.
        while let Some(i) = (0..self.count).find(|&i| self.rects[i].touches(r)) {
            r = r.union(self.take(i));
        }
        if self.count < MAX_RECTS {
            self.rects[self.count] = r;
            self.count += 1;
            return;
        }
        // Full, and touching none: fold into whichever grows the least. The
        // region stays covered, a little less tightly, and the fold may now
        // touch others, so it goes round again.
        let j = self.smallest_growth(r);
        let grown = r.union(self.take(j));
        self.accumulate(grown);
    }

    /// Remove the held rectangle at `i`; the last takes its place.
    fn take(&mut self, i: usize) -> Rect {
        let r = self.rects[i];
        self.count -= 1;
        self.rects[i] = self.rects[self.count];
        r
    }

    fn smallest_growth(&self, r: Rect) -> usize {
        let growth = |i: &usize| self.rects[*i].union(r).area() - self.rects[*i].area();
        (0..self.count).min_by_key(growth).unwrap_or(0)
    }
}
