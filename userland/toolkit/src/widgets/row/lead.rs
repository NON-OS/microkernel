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
//! An icon leading a row's label: the builder, and the square it takes from
//! the left of the text lane.

use super::Row;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::{ICON, SPACE};
use crate::icons::IconId;
use crate::widgets::rect::{centre, split_left};

impl Row<'_> {
    /// An icon before the label, as a navigation item draws one: an `ICON`
    /// square in muted ink, or disabled ink on a disabled row. The label and
    /// note start `SPACE.md` after it.
    pub fn icon(mut self, id: IconId) -> Self {
        self.icon = Some(id);
        self
    }

    /// The icon's square, centred down the lane at its left end, and the lane
    /// left for the text. Without an icon there is no square and the lane is
    /// returned as given.
    pub(super) fn lead(&self, lane: Rect) -> (Option<Rect>, Rect) {
        if self.icon.is_none() {
            return (None, lane);
        }
        let s = scale();
        let (pad, side) = (s.px(u32::from(SPACE.md)), s.px(ICON).min(lane.h));
        let (slot, after) = split_left(lane, side);
        let (_, text) = split_left(after, pad);
        (Some(centre(slot, side, side)), text)
    }
}
