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

//! A pill's natural size, which its size and its layout share.

use super::Pill;
use crate::design::scale::scale;
use crate::design::size::{DOT, ICON, PILL_H, PILL_PAD, SPACE};
use crate::design::text_role::TextRole;
use crate::widgets::text;

impl Pill<'_> {
    /// The label's width plus `PILL_PAD` each side, plus the icon or dot and
    /// a `SPACE.xs` gap when there is one, plus any count and the `SPACE.sm`
    /// gap before it, by `PILL_H`.
    pub fn size(&self) -> (u32, u32) {
        let s = scale();
        let pad = s.px(PILL_PAD).saturating_mul(2);
        let w = text::width(self.label, Self::px()).saturating_add(pad);
        let w = w.saturating_add(self.lead()).saturating_add(self.count_w());
        (w, s.px(PILL_H))
    }

    /// The body text size the label is drawn at.
    pub(super) fn px() -> f32 {
        scale().font(TextRole::Body.px())
    }

    /// The width the icon, else the dot, and its gap take before the label;
    /// 0 with neither, and a filled pill never has a dot.
    pub(super) fn lead(&self) -> u32 {
        let s = scale();
        let gap = s.px(u32::from(SPACE.xs));
        if self.icon.is_some() {
            return s.px(ICON).saturating_add(gap);
        }
        if !self.dot || self.filled {
            return 0;
        }
        s.px(DOT).saturating_add(gap)
    }
}
