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

//! A pill's count: how wide it is and where it sits after the label.

use super::Pill;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::SPACE;
use crate::widgets::digits::Digits;
use crate::widgets::text;

impl Pill<'_> {
    /// The width the count and the `SPACE.sm` gap before it add; 0 without one.
    pub(super) fn count_w(&self) -> u32 {
        self.count.map_or(0, |n| {
            let gap = scale().px(u32::from(SPACE.sm));
            text::width(Digits::of(n).as_str(), Self::px()).saturating_add(gap)
        })
    }

    /// Splits the count's box off the right end of the label's `lane`. With
    /// no count, or too little lane for all of it, the lane is returned whole
    /// and there is no box.
    pub(super) fn tail(&self, lane: Rect) -> (Rect, Option<Rect>) {
        let need = self.count_w();
        if need == 0 || need > lane.w {
            return (lane, None);
        }
        let cw = need - scale().px(u32::from(SPACE.sm));
        let count = Rect { x: lane.x.saturating_add(lane.w - cw), w: cw, ..lane };
        (Rect { w: lane.w - need, ..lane }, Some(count))
    }
}
