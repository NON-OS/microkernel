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

use crate::browser::css::{Float, Size};

use super::super::border_box_w::border_box_w;
use super::super::ctx::Ctx;
use super::super::display_list::DisplayList;
use super::super::float_ctx::FloatCtx;
use super::super::geom::ctx::Pin;
use super::super::geom::margins::margins;
use super::super::layout_box::layout_box;
use super::super::post::max_content::shrink_to_fit;
use super::super::tree::BoxNode;

/// Lay a float found at flow y `cy` in a container `w` px wide. It leaves
/// normal vertical flow: sized, dropped to the row where it fits beside the
/// existing floats, and recorded so following content wraps around it.
/// Its margins count toward the room it takes, a negative one shrinking
/// it, which is how a margin-left:-100% sidebar lands beside its column.
pub(crate) fn layout_float(
    child: &BoxNode,
    floats: &mut FloatCtx,
    cy: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) {
    let cs = &child.style;
    let [mt, mr, mb, ml] = margins(cs, w);
    let is_left = cs.float == Float::Left;
    let clear_y = floats.clear_row(cs.clear, cy);
    let fw = float_width(child, w, w - ml - mr, depth, ctx);
    let outer = fw + ml + mr;
    let (fx, fy) = floats.next_pos(is_left, outer, clear_y);
    /* The float takes exactly the width sized here, a percentage one
     * included, rather than resolving its width again against it. */
    let fctx = Ctx { pin: Some(Pin { w: fw, h: None }), ..ctx };
    let fh = layout_box(child, fx + ml, fy + mt, fw, frags, depth + 1, fctx);
    floats.record(is_left, fx, outer, fy, fy + mt + fh + mb);
}

/* A float's border-box width: its declared width against the container's
 * `w`, or when auto, shrink-to-fit in the `room` its margins leave. */
fn float_width(node: &BoxNode, w: i32, room: i32, depth: u32, ctx: Ctx) -> i32 {
    match node.style.width {
        Size::Auto => border_box_w(&node.style, shrink_to_fit(node, room, depth, ctx)),
        _ => border_box_w(&node.style, w.max(0)),
    }
}
