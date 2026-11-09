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

//! Where a pill's body, dot and label sit, which draw and hit share.

use super::Pill;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::{DOT, ICON, PILL_PAD};
use crate::widgets::rect::{centre, split_left};
use crate::widgets::text;

/// The placed parts of a pill, in drawing pixels. `dot` is the centre and
/// radius of the status dot, if there is one; `icon` the icon's square;
/// `count` the count's box.
pub(super) struct Parts {
    pub(super) body: Rect,
    pub(super) radius: u32,
    pub(super) dot: Option<(u32, u32, u32)>,
    pub(super) icon: Option<Rect>,
    pub(super) label: Rect,
    pub(super) count: Option<Rect>,
    pub(super) label_top: i32,
    pub(super) px: f32,
}

impl Pill<'_> {
    /// The body is the pill's size, clamped to `at`, at the left edge of `at`
    /// and centred vertically in it, with a radius of half its height. The
    /// dot, or the icon in its place, sits `PILL_PAD` in from the left,
    /// centred vertically; the label box follows it and stops `PILL_PAD`
    /// short of the right edge, less any count's box at that end.
    pub(super) fn parts(&self, at: Rect) -> Parts {
        let s = scale();
        let (w, h) = self.size();
        let body = Rect { x: at.x, ..centre(at, w, h) };
        let pad = s.px(PILL_PAD);
        let inner = Rect {
            x: body.x.saturating_add(pad),
            w: body.w.saturating_sub(pad.saturating_mul(2)),
            ..body
        };
        let (lead, label) = split_left(inner, self.lead());
        let (label, count) = self.tail(label);
        let d = s.px(DOT);
        let cy = body.y.saturating_add(body.h / 2);
        let dot = (self.lead() > 0).then(|| (lead.x.saturating_add(d / 2), cy, d / 2));
        let dot = dot.filter(|_| self.icon.is_none());
        let side = s.px(ICON).min(body.h).min(lead.w);
        let icon = self.icon.map(|_| centre(Rect { w: side, ..lead }, side, side));
        let px = Self::px();
        let label_top = text::centre_top(body.y, body.h, px);
        Parts { body, radius: body.h / 2, dot, icon, label, count, label_top, px }
    }
}
