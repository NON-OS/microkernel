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

//! A canvas rectangle and the geometry the accumulator and the bound use.
//! Edges saturate: a client's u32s may sum past the top, and the canvas is
//! far smaller than that, so a saturated edge is still past every pixel.

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn right(self) -> u32 {
        self.x.saturating_add(self.width)
    }

    pub fn bottom(self) -> u32 {
        self.y.saturating_add(self.height)
    }

    pub fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn area(self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// The two overlap or share an edge, so one rectangle covering both
    /// composes no pixel the two did not.
    pub fn touches(self, b: Rect) -> bool {
        self.x <= b.right() && b.x <= self.right() && self.y <= b.bottom() && b.y <= self.bottom()
    }

    /// The smallest rectangle holding both.
    pub fn union(self, b: Rect) -> Rect {
        let (x0, y0) = (self.x.min(b.x), self.y.min(b.y));
        let (x1, y1) = (self.right().max(b.right()), self.bottom().max(b.bottom()));
        Rect { x: x0, y: y0, width: x1 - x0, height: y1 - y0 }
    }

    /// The pixels in both, or None when they share none.
    pub fn intersect(self, b: Rect) -> Option<Rect> {
        let (x0, y0) = (self.x.max(b.x), self.y.max(b.y));
        let (x1, y1) = (self.right().min(b.right()), self.bottom().min(b.bottom()));
        (x0 < x1 && y0 < y1).then(|| Rect { x: x0, y: y0, width: x1 - x0, height: y1 - y0 })
    }
}
