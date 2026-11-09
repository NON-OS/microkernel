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

//! A tab's lead: the icon before its label, or the busy dot in its place.

use super::Tabs;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::{DOT, ICON, SPACE};
use crate::theme::{roles, Opaque};
use crate::widgets::rect::{centre, split_left};
use crate::widgets::{draw, Surface};

impl Tabs<'_> {
    /// Whether tab `i` is busy.
    fn is_busy(&self, i: usize) -> bool {
        self.busy.get(i).copied().unwrap_or(false)
    }

    /// The width tab `i`'s lead and the gap after it add: none without one.
    pub(super) fn lead_w(&self, i: usize) -> u32 {
        if self.icons.get(i).is_none() && !self.is_busy(i) {
            return 0;
        }
        let s = scale();
        s.px(ICON).saturating_add(s.px(u32::from(SPACE.sm)))
    }

    /// The lead's square, centred down the left end of `lane`, and the lane
    /// left for the label. Without a lead the lane is returned as given.
    pub(super) fn lead(&self, i: usize, lane: Rect) -> (Option<Rect>, Rect) {
        if self.lead_w(i) == 0 {
            return (None, lane);
        }
        let s = scale();
        let side = s.px(ICON).min(lane.h);
        let (slot, after) = split_left(lane, side);
        let (_, label) = split_left(after, s.px(u32::from(SPACE.sm)));
        (Some(centre(slot, side, side)), label)
    }

    /// Paints tab `i`'s lead in `sq`: the busy dot in `accent_text`, or
    /// `disabled` on a disabled strip, else the icon in `ink`.
    pub(super) fn draw_lead(&self, fb: &mut impl Surface, i: usize, sq: Rect, ink: Opaque) {
        let side = sq.w.min(sq.h);
        if self.is_busy(i) {
            let r = roles();
            let dot = if self.state.disabled { r.disabled } else { r.accent_text };
            let rad = (scale().px(DOT) / 2).min(side / 2);
            draw::dot(fb, sq.x.saturating_add(sq.w / 2), sq.y.saturating_add(sq.h / 2), rad, dot);
        } else if let Some(&id) = self.icons.get(i) {
            draw::icon(fb, id, sq.x, sq.y, side, ink);
        }
    }
}
