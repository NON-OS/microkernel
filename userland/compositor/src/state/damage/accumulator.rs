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

// Damage is tracked as a small set of separate rectangles rather than one
// bounding box. With a window open, damage arrives from far-apart places (the
// cursor, an app window, the top bar); merging them into one box made the
// compositor recomposite the whole screen, including the empty space between,
// every frame. Keeping them separate means only the pixels that actually
// changed are recomposited. Rectangles that overlap or touch are coalesced
// (accumulate.rs), so no pixel is composed twice, and the set is bounded.

use super::rect::Rect;

pub(super) const MAX_RECTS: usize = 8;

pub struct DamageAccumulator {
    pub(super) rects: [Rect; MAX_RECTS],
    pub(super) count: usize,
}

impl DamageAccumulator {
    pub const fn new() -> Self {
        const EMPTY: Rect = Rect { x: 0, y: 0, width: 0, height: 0 };
        Self { rects: [EMPTY; MAX_RECTS], count: 0 }
    }

    pub fn mark_full(&mut self, width: u32, height: u32) {
        self.rects[0] = Rect { x: 0, y: 0, width, height };
        self.count = 1;
    }

    /// One damaged rectangle per call; the compositor loop drains the rest on
    /// following iterations.
    pub fn drain(&mut self) -> Option<Rect> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        Some(self.rects[self.count])
    }
}

impl Default for DamageAccumulator {
    fn default() -> Self {
        Self::new()
    }
}
