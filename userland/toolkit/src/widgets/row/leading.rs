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
//! A box at a row's left end kept free for the app's own drawing, such as a
//! letter tile or a file-type tile: the builder, the box, and the lane it
//! leaves for the icon and text.

use super::Row;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::SPACE;
use crate::widgets::rect::split_left;

impl Row<'_> {
    /// Keeps a box `w` drawing pixels wide free at the left, the row's full
    /// height, for a tile the app draws itself. Any icon, the label and the
    /// note start `SPACE.md` after it, as they start after an icon. The row's
    /// washes and focus ring still cover it. `leading_rect` gives the box.
    pub fn leading(mut self, w: u32) -> Self {
        self.leading = w;
        self
    }

    /// The box `leading` keeps free, the row's full height, where the app
    /// draws its tile. Zero wide when the row has no leading box.
    pub fn leading_rect(&self, at: Rect) -> Rect {
        self.parts(at).leading
    }

    /// Splits the leading box off the left of `lane`, clamped to it, and
    /// the lane left after it and its gap. Without a leading box the box is
    /// zero wide at the lane's left and the lane is returned as given.
    pub(super) fn leading_box(&self, lane: Rect) -> (Rect, Rect) {
        if self.leading == 0 {
            return (Rect { w: 0, ..lane }, lane);
        }
        let (slot, after) = split_left(lane, self.leading);
        let (_, rest) = split_left(after, scale().px(u32::from(SPACE.md)));
        (slot, rest)
    }
}
