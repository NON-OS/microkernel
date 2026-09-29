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

use crate::browser::css::Position;

use super::abs_out_of_flow::positioned;
use super::box_kind::inner;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::flow::layout_image::layout_image;
use super::geom::rel_offset::rel_offset;
use super::layout_block::layout_block;
use super::layout_flex::layout_flex;
use super::layout_grid::layout_grid;
use super::post::apply_fx::apply_fx;
use super::post::place_out::place_inside;
use super::tree::{BoxKind, BoxNode};

/* Route a block-level box to its formatting context. position:relative
 * draws the box shifted while its returned flow height keeps siblings where
 * normal flow puts them. A positioned box then places the out-of-flow boxes
 * it contains, and a transform or clip-path applies to all it painted. */
pub(super) fn layout_box(
    node: &BoxNode,
    x: i32,
    y: i32,
    avail: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let ctx = ctx.enter(&node.style, y);
    let (mut x, mut y) = (x, y);
    if node.style.position == Position::Relative {
        let (dx, dy) = rel_offset(&node.style, ctx.cb);
        x += dx;
        y += dy;
    }
    let start = frags.len();
    let h = route(node, x, y, avail, frags, depth, ctx);
    if positioned(&node.style) {
        place_inside(node, frags, start, depth, ctx);
    }
    apply_fx(&node.style, frags, start, ctx.clip);
    h
}

fn route(n: &BoxNode, x: i32, y: i32, w: i32, f: &mut DisplayList, d: u32, ctx: Ctx) -> i32 {
    if n.style.is_table {
        /* A table sizes its own cells; a pinned width is its width. */
        let w = ctx.pin.map_or(w, |p| p.w);
        return super::layout_table::layout_table(n, x, y, w, f, d, Ctx { pin: None, ..ctx });
    }
    match inner(n) {
        BoxKind::Flex => layout_flex(n, x, y, w, f, d, ctx),
        BoxKind::Grid => layout_grid(n, x, y, w, f, d, ctx),
        /* An image laid as a box of its own, as an absolutely positioned
         * one is, still paints its picture. */
        BoxKind::Image { .. } => layout_image(n, x, y, w, f, ctx),
        _ => layout_block(n, x, y, w, f, d, ctx),
    }
}
