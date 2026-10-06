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

use crate::browser::css::Size;

use super::super::border_box_w::border_box_w;
use super::super::edges_x::edges_x;
use super::super::geom::margins::margins;
use super::super::replaced_size::replaced_size;
use super::super::tree::{BoxKind, BoxNode};
use super::max_content::shrink_to_fit;
use super::place_one::Cb;

/// The border-box width of an out-of-flow box in the containing block `cb`
/// with resolved left and right insets `lr`, and the width it is
/// placed within (wider only when auto margins centre it between both
/// insets). An auto width fills the gap between two set insets; with one or
/// none it shrinks to fit its content in the room left, from the static x
/// `sx` when neither is set. An image keeps its own size.
pub(crate) fn place_width(
    c: &BoxNode,
    cb: &Cb,
    lr: [Option<i32>; 2],
    sx: i32,
    d: u32,
) -> (i32, i32) {
    let s = &c.style;
    let ([l, r], ctx, cb) = (lr, cb.ctx, cb.r);
    let cw = cb[2];
    let [_, mr, _, ml] = margins(s, cw);
    let gap = cw - l.unwrap_or(0) - r.unwrap_or(0) - ml - mr;
    let both = l.is_some() && r.is_some();
    if s.width != Size::Auto {
        let w = border_box_w(s, cw);
        return (w, if both { gap.max(w) } else { w });
    }
    if both {
        let w = border_box_w(s, gap.max(0));
        return (w, w);
    }
    let room = match (l, r) {
        (Some(l), _) => cw - l,
        (None, Some(r)) => cw - r,
        (None, None) => cb[0] + cw - sx,
    };
    let room = (room - ml - mr).max(0);
    let w = if let BoxKind::Image { .. } = c.kind {
        let (el, er) = edges_x(s);
        replaced_size(c, room, Some(cb[3])).0 + el + er
    } else {
        shrink_to_fit(c, room, d, ctx)
    };
    let w = border_box_w(s, w);
    (w, w)
}
