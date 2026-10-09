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

use crate::browser::css::Computed;

use super::super::ctx::{Ctx, Pin};
use super::super::display_list::DisplayList;
use super::super::geom::margins::margins;
use super::super::layout_box::layout_box;
use super::super::tree::BoxNode;
use super::cross_fit::fit_across;
use super::grid_align::col_align;

/* A grid item laid in its area's columns `r` ([x, top, width]). It fills
 * the area's width (justify-self stretch, the default for a box), or
 * with start, center or end alignment, or auto margins, fits its content
 * and sits where they put it (cross_fit.rs). Its width is pinned, so a
 * percentage one is taken of the area once. Returns its fragments [a, b),
 * border-box height and vertical margins, for the row pass. */
pub(in super::super) fn lay_item(
    it: &BoxNode,
    r: [i32; 3],
    s: &Computed,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> ([usize; 2], i32, [i32; 2]) {
    let st = &it.style;
    let [mt, mr, mb, ml] = margins(st, r[2]);
    let room = (r[2] - ml - mr).max(0);
    let (bw, dx) = fit_across(it, room, col_align(it, s), s.rtl, depth);
    let start = frags.len();
    let pinned = Ctx { pin: Some(Pin { w: bw, h: None }), ..ctx };
    let h = layout_box(it, r[0] + ml + dx, r[1] + mt, bw, frags, depth + 1, pinned);
    ([start, frags.len()], h, [mt, mb])
}
