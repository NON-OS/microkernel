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

use super::auto_repeat_n::auto_repeat_n;
use super::box_kind::is_item;
use super::contexts::grid_align::row_align;
use super::contexts::grid_areas::areas;
use super::contexts::grid_cols::col_sizes;
use super::contexts::grid_item::lay_item;
use super::contexts::grid_rows::row_sizes;
use super::contexts::self_align::cross_shift;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::shift_down::shift_down;
use super::tree::BoxNode;

/* Lay a grid's items into its content box at (x, y), `w` wide: place them
 * (grid_areas.rs), size the columns for the items in them, lay each item
 * at the top in its columns, size the rows for the heights that gave, and
 * drop each item into its rows, aligned there. Columns run right to left
 * in an rtl grid. Returns the height of the rows. */
pub(super) fn grid_children(
    node: &BoxNode,
    x: i32,
    y: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let s = &node.style;
    let items: Vec<&BoxNode> = node.children.iter().filter(|c| is_item(c)).collect();
    let n_cols = auto_repeat_n(s, w, items.len()).unwrap_or(s.grid_col_n as usize);
    let (placed, ncols, nrows) = areas(&items, n_cols, s);
    let cols = col_sizes(s, &items, &placed, w, [n_cols, ncols], depth);
    let mut laid = Vec::with_capacity(items.len());
    for (it, a) in items.iter().zip(&placed) {
        let (c0, c1) = (cols[a.c.min(ncols - 1)], cols[(a.c + a.cs - 1).min(ncols - 1)]);
        let area_w = c1.0 + c1.1 - c0.0;
        let ax = if s.rtl { w - c1.0 - c1.1 } else { c0.0 };
        laid.push(lay_item(it, [x + ax, y, area_w], s, frags, depth, ctx));
    }
    let heights = laid.iter().map(|l| l.1 + l.2[0] + l.2[1]);
    let (rows, total) = row_sizes(s, &placed, heights, nrows, ctx.cb.h);
    for ((it, a), (ab, h, [mt, mb])) in items.iter().zip(&placed).zip(laid) {
        let (r0, r1) = (rows[a.r.min(nrows - 1)], rows[(a.r + a.rs - 1).min(nrows - 1)]);
        shift_down(frags, ab[0], ab[1], r0.0, ctx.clip);
        cross_shift(frags, ab, [mt, h, mb], r1.0 + r1.1 - r0.0, row_align(it, s), ctx.clip);
    }
    total
}
