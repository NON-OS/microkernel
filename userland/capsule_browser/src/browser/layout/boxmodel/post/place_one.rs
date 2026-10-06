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

use super::super::ctx::{Ctx, Pin};
use super::super::display_list::DisplayList;
use super::super::geom::containing::Containing;
use super::super::geom::margins::margins;
use super::super::layout_box::layout_box;
use super::super::shift_down::shift_down;
use super::super::tree::BoxNode;
use super::place_width::place_width;

/// The containing block an out-of-flow box is placed against: a padding
/// box [x, y, w, h] in page coordinates, and the layout state inside it.
#[derive(Clone, Copy)]
pub(crate) struct Cb {
    pub r: [i32; 4],
    pub ctx: Ctx,
}

/// Lay one absolutely positioned or fixed box against its containing block.
/// A side whose inset is set anchors the box there (percentages of the
/// block's width across, of its height down); an axis with both insets
/// auto keeps the static position flow recorded. An auto height between a
/// set top and bottom fills the gap; with only bottom set, the box is laid
/// and then dropped so its bottom margin edge meets the inset.
pub(crate) fn place_one(c: &BoxNode, cb: Cb, frags: &mut DisplayList, d: u32) {
    let s = &c.style;
    let [cx, cy, cw, ch] = cb.r;
    let (sx, sy, flow) = c.aux.flow_at.get().unwrap_or((cx, cy, cb.ctx));
    let mut ctx = cb.ctx;
    (ctx.z, ctx.alpha, ctx.fixed, ctx.sticky) = (flow.z, flow.alpha, flow.fixed, flow.sticky);
    ctx.cb = Containing { w: cw, h: Some(ch) };
    let (l, r) = (s.left.resolve(cw), s.right.resolve(cw));
    let (t, b) = (s.top.resolve(ch), s.bottom.resolve(ch));
    let [mt, mr, mb, ml] = margins(s, cw);
    let (w, avail) = place_width(c, &cb, [l, r], sx, d);
    let x = match (l, r) {
        (Some(l), _) => cx + l + ml,
        (None, Some(r)) => cx + cw - r - mr - w,
        (None, None) => sx + ml,
    };
    let pin_h = match (s.height, t, b) {
        (Size::Auto, Some(t), Some(b)) if s.aspect.is_none() => Some((ch - t - b - mt - mb).max(0)),
        _ => None,
    };
    ctx.pin = Some(Pin { w, h: pin_h });
    let y = cy.saturating_add(t.unwrap_or(sy - cy)).saturating_add(mt);
    let start = frags.len();
    let h = layout_box(c, x, y, avail, frags, d, ctx);
    if let (None, Some(b)) = (t, b) {
        let dy = cy + ch - b - mb - h - y;
        let end = frags.len();
        shift_down(frags, start, end, dy, ctx.clip);
    }
}
