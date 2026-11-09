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

//! A navigation item's trailing count: where its digits sit.

use super::NavItem;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::SPACE;
use crate::widgets::digits::Digits;
use crate::widgets::text;

impl NavItem<'_> {
    /// The width the count and the gap before it add: none without one.
    pub(super) fn count_w(&self, px: f32) -> u32 {
        self.count.map_or(0, |n| {
            let pad = scale().px(u32::from(SPACE.md));
            text::width(Digits::of(n).as_str(), px).saturating_add(pad)
        })
    }

    /// Splits the count's box off the right end of `lane`, `SPACE.md` from
    /// the label's. Without a count, or with too little lane for all of
    /// it, the lane is returned whole and there is no box.
    pub(super) fn tail(&self, lane: Rect, px: f32) -> (Rect, Option<Rect>) {
        let need = self.count_w(px);
        if need == 0 || need > lane.w {
            return (lane, None);
        }
        let cw = need - scale().px(u32::from(SPACE.md));
        let count = Rect { x: lane.x.saturating_add(lane.w - cw), w: cw, ..lane };
        (Rect { w: lane.w - need, ..lane }, Some(count))
    }
}
