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

//! Painting and hit testing a pill, both from `parts`.

use super::Pill;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::theme::roles;
use crate::widgets::digits::Digits;
use crate::widgets::rect::contains;
use crate::widgets::{draw, text, Surface};

impl Pill<'_> {
    /// Paints the `raised` body, or the category's on a filled pill, its
    /// outline, the dot or icon, the label, any count and the focus ring. Selected
    /// adds `accent_wash` over the body and outlines in `accent_edge`, else
    /// the outline is `border_strong`; hovered adds `hover_wash`; pressed
    /// moves the body away from the label. A filled pill takes no wash; it
    /// moves its fill instead. Disabled draws the outline, dot, label and
    /// count in `disabled` and ignores the other states.
    pub fn draw(&self, fb: &mut impl Surface, at: Rect) {
        let (p, r, s, st) = (self.parts(at), roles(), scale(), self.state);
        let live = !st.disabled;
        draw::round(fb, p.body, p.radius, self.body_fill(&r));
        let wash = live && self.fill_by().is_none();
        if wash && st.selected {
            draw::round(fb, p.body, p.radius, r.accent_wash);
        }
        if wash && st.hovered {
            draw::round(fb, p.body, p.radius, r.hover_wash);
        }
        if live && st.selected {
            draw::outline(fb, p.body, p.radius, s.hair(), r.accent_edge);
        } else {
            let edge = if live { r.border_strong } else { r.disabled };
            draw::outline(fb, p.body, p.radius, s.hair(), edge);
        }
        if let Some((cx, cy, rad)) = p.dot {
            draw::dot(fb, cx, cy, rad, self.dot_ink(&r));
        }
        if let Some((id, sq)) = self.icon.zip(p.icon) {
            draw::icon(fb, id, sq.x, sq.y, sq.w.min(sq.h), self.label_ink(&r));
        }
        let label = text::truncate(self.label, p.label.w, p.px);
        text::left(fb, p.label.x, p.label_top, &label, self.label_ink(&r), p.px);
        if let Some((n, at)) = self.count.zip(p.count) {
            text::left(fb, at.x, p.label_top, Digits::of(n).as_str(), self.count_ink(&r), p.px);
        }
        if live && st.focused {
            draw::outline(fb, p.body, p.radius, s.hair().saturating_mul(2), r.focus_ring);
        }
    }

    /// Whether the point lies on the pill's body.
    /// `at` and the point use `draw`'s coordinates: content coordinates when
    /// drawn through `Scrolled`, so pass the pointer's y plus the scroll.
    pub fn hit(&self, at: Rect, x: i32, y: i32) -> bool {
        contains(self.parts(at).body, x, y)
    }
}
