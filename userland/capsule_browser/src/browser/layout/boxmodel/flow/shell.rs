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

use super::super::border_box_w::border_box_w;
use super::super::ctx::Ctx;
use super::super::display_list::{DisplayList, Fragment};
use super::super::edges_x::edges_x;
use super::super::edges_y::edges_y;
use super::super::geom::containing::Containing;
use super::super::geom::fixed_h::{def_h, min_max_h};
use super::super::geom::overflow_clip::overflow_clip;
use super::super::tree::BoxNode;
use super::static_pos::record_static;

/// A block, flex or grid container's border box and its children's context.
pub(crate) struct Shell {
    pub cx: i32,
    pub cy: i32,
    pub cw: i32,
    /// Children's context: this content box, clipped by its overflow.
    pub inner: Ctx,
    def_h: Option<i32>,
    floor: bool,
    ey: i32,
    cb_h: Option<i32>,
    slot: usize,
}

/// Size the box at `r` ([x, y, available width]; or the border box its
/// insets pinned) and push its own fragment, height to follow at close.
pub(crate) fn open(n: &BoxNode, r: [i32; 3], frags: &mut DisplayList, ctx: Ctx) -> Shell {
    let ([x, y, avail], s) = (r, &n.style);
    let w = ctx.pin.map_or_else(|| border_box_w(s, avail), |p| p.w);
    /* margin:auto on both sides centres a box narrower than its space. */
    let x = if s.margin_auto_x && w < avail { x + (avail - w) / 2 } else { x };
    let ((el, er), (et, eb)) = (edges_x(s), edges_y(s));
    let (cw, ey) = ((w - el - er).max(0), et + eb);
    let (def_h, floor) = def_h(s, ctx.pin.and_then(|p| p.h), ctx.cb.h, [w, cw, ey]);
    let cb = Containing { w: cw, h: def_h.map(|h| (h - ey).max(0)) };
    let mut inner = Ctx { pin: None, cb, ..ctx };
    if let Some(clip) = overflow_clip(s, [x, y, w], def_h) {
        inner = inner.clipped(clip);
    }
    record_static(&n.children, x + el, y + et, inner);
    let slot = frags.len();
    frags.push(Fragment::of_box(n, [x, y, w, 0], &ctx));
    Shell { cx: x + el, cy: y + et, cw, inner, def_h, floor, ey, cb_h: ctx.cb.h, slot }
}

impl Shell {
    /// Settle the height (definite, a ratio's grown to its content, else
    /// content plus edges; then clamped) and write it into the fragment.
    pub(crate) fn close(self, n: &BoxNode, inner_h: i32, frags: &mut DisplayList) -> i32 {
        let content = inner_h + self.ey;
        let h = self.def_h.map_or(content, |d| if self.floor { d.max(content) } else { d });
        let h = min_max_h(&n.style, h, self.cb_h, self.ey);
        if let Some(f) = frags.get_mut(self.slot) {
            f.h = h;
        }
        h
    }
}
