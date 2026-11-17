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

//! Where a button's icon sits, alone or leading its label.

use super::Button;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::SPACE;
use crate::widgets::rect::centre;
use crate::widgets::text;

impl Button<'_> {
    /// The label's width, or with an icon the icon's side, and with both the
    /// side, `SPACE.sm` and the label's width.
    pub(super) fn content_w(&self, px: f32) -> u32 {
        let (s, tw) = (scale(), text::width(self.label, px));
        let lead = s.px(self.side).saturating_add(s.px(u32::from(SPACE.sm)));
        match (self.icon.is_some(), self.label.is_empty()) {
            (false, _) => tw,
            (true, true) => s.px(self.side),
            (true, false) => lead.saturating_add(tw),
        }
    }

    /// The icon's square and the label's box within `inner`, the body less its
    /// padding. With no icon the label box is `inner`. With no label, or no
    /// room for one, the icon is centred in the body. Otherwise the icon,
    /// `SPACE.sm` and the label, cut to what is left, are centred in `inner`
    /// as one group. The icon shrinks to stay square inside the body.
    pub(super) fn place(&self, at: Rect, inner: Rect, px: f32) -> (Option<Rect>, Rect) {
        if self.icon.is_none() {
            return (None, inner);
        }
        let s = scale();
        let (side, gap) = (s.px(self.side).min(at.h), s.px(u32::from(SPACE.sm)));
        let lead = side.saturating_add(gap);
        if self.label.is_empty() || inner.w <= lead {
            let side = side.min(at.w);
            return (Some(centre(at, side, side)), Rect { w: 0, ..inner });
        }
        let tw = text::width(self.label, px).min(inner.w - lead);
        let x = inner.x.saturating_add((inner.w - lead - tw) / 2);
        let icon = Rect { x, ..centre(at, side, side) };
        (Some(icon), Rect { x: x.saturating_add(lead), w: tw, ..inner })
    }
}
