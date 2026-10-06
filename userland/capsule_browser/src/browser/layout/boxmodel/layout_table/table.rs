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

use alloc::vec::Vec;

use super::super::ctx::Ctx;
use super::super::display_list::{DisplayList, Fragment};
use super::super::edges_x::edges_x;
use super::super::edges_y::edges_y;
use super::super::geom::containing::Containing;
use super::super::layout_block::{layout_block, MAX_DEPTH};
use super::super::layout_box::layout_box;
use super::super::tree::BoxNode;
use super::{grid, place, width};

/* The table laid out at `r` ([x, y, available width]). */
pub(super) fn lay(
    node: &BoxNode,
    r: [i32; 3],
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let [x, y, avail] = r;
    let g = grid::grid(node);
    if g.slots.is_empty() || depth > MAX_DEPTH {
        return layout_block(node, x, y, avail, frags, depth, ctx);
    }
    let s = &node.style;
    let ((el, er), (et, eb), sp) = (edges_x(s), edges_y(s), width::spacing(node));
    let width::Sized { w, ws } = width::size(node, &g, avail, depth);
    let centre = (s.margin_auto_x || s.table.center_blocks) && w < avail;
    let x = if centre { x + (avail - w) / 2 } else { x };
    let slot = frags.len();
    frags.push(Fragment::of_box(node, [x, y, w, 0], &ctx));
    let cb = Containing { w: (w - el - er).max(0), h: None };
    let ctx = Ctx { pin: None, cb, ..ctx };
    let mut cy = y + et;
    for c in &g.captions {
        cy += layout_box(c, x + el, cy, cb.w, frags, depth + 1, ctx);
    }
    let mut xs = Vec::with_capacity(ws.len());
    ws.iter().fold(x + el + sp, |at, cw| {
        xs.push(at);
        at + cw + sp
    });
    let row = (x + el + sp, (cb.w - 2 * sp).max(0));
    let bottom = place::place(&g, &place::Geo { xs, ws, sp, row, y: cy }, frags, depth, ctx);
    let h = (bottom - y + eb).max(s.height.definite_px().unwrap_or(0));
    if let Some(f) = frags.get_mut(slot) {
        f.h = h;
    }
    h
}
