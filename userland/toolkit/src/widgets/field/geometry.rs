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

//! Where a field's body, icon, text and caret sit, and which part of its
//! text is shown, which draw and hit share.

use super::view::View;
use super::Field;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::{CONTROL_RADIUS, FIELD_ICON, SPACE};
use crate::font::ttf::line_height;
use crate::widgets::rect::{centre, split_left};
use crate::widgets::text;

/// The placed parts of a field, in drawing pixels.
pub(super) struct Parts {
    pub(super) body: Rect,
    pub(super) radius: u32,
    pub(super) icon: Option<Rect>,
    pub(super) text: Rect,
    pub(super) top: i32,
    pub(super) px: f32,
    pub(super) view: View,
    pub(super) caret: Option<Rect>,
    pub(super) selection: Option<Rect>,
}

impl Field<'_> {
    /// The body is `at` as given. Any icon is a `FIELD_ICON` square
    /// `SPACE.md` in from the left, centred vertically. The text box runs
    /// from `SPACE.sm` after the icon, or `SPACE.md` in without one, to
    /// `SPACE.md` short of the right edge, with one line at the role's size
    /// centred vertically. The caret is a hairline the height of the line box, at
    /// the end of the shown text up to the caret, kept inside the text box.
    /// A selection spans the line box between where the caret would stand
    /// at either end of it.
    pub(super) fn parts(&self, at: Rect) -> Parts {
        let s = scale();
        let (pad, px) = (s.px(u32::from(SPACE.md)), s.font(self.role.px()));
        let (side, gap) = (s.px(FIELD_ICON).min(at.h), s.px(u32::from(SPACE.sm)));
        let (_, inner) = split_left(at, pad);
        let (slot, after) = split_left(inner, self.icon.map_or(0, |_| side));
        let icon = self.icon.map(|_| centre(slot, side, side));
        let (_, room) = split_left(after, icon.map_or(0, |_| gap));
        let room = Rect { w: room.w.saturating_sub(pad), ..room };
        let top = text::centre_top(at.y, at.h, px);
        let view = self.view(room.w, px);
        let line = Rect {
            x: room.x,
            y: u32::try_from(top).unwrap_or(at.y),
            w: s.hair(),
            h: (line_height(px).max(0) as u32).min(at.h),
        };
        let x = |c: usize| room.x.saturating_add(reach(&view.shown, view.shown_at(c), room.w, px));
        let caret = self.caret.filter(|_| self.typing()).map(|c| self.boundary(c));
        let caret = caret.map(|c| Rect { x: x(c), ..line });
        let ends = self.selection.filter(|_| self.typing());
        let ends = ends.map(|(a, b)| (self.boundary(a), self.boundary(b)));
        let selection = ends.map(|(a, b)| (x(a.min(b)), x(a.max(b))));
        let selection = selection.map(|(l, r)| Rect { x: l, w: r.saturating_sub(l), ..line });
        let radius = s.px(CONTROL_RADIUS);
        Parts { body: at, radius, icon, text: room, top, px, view, caret, selection }
    }
}

/// How far into a text box `w` wide the caret stands after the first `k`
/// bytes of `shown`: their width, kept a hairline short of the far edge.
pub(super) fn reach(shown: &str, k: usize, w: u32, px: f32) -> u32 {
    let far = w.saturating_sub(scale().hair());
    text::width(shown.get(..k).unwrap_or(""), px).min(far)
}
