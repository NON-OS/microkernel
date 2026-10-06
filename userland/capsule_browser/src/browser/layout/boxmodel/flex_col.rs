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

use super::box_kind::is_item;
use super::contexts::cross_fit::fit_across;
use super::contexts::flex_col_free::{spread, Laid};
use super::ctx::{Ctx, Pin};
use super::display_list::DisplayList;
use super::geom::margins::margins;
use super::layout_box::layout_box;
use super::tree::BoxNode;

/* A flex column: items stack top to bottom with the row gap between them.
 * Across, an item of auto width stretches to the column (align-self or
 * align-items stretch) or else fits its content, and sits where its
 * alignment or auto margins put it; its width is pinned. Free height in a
 * box of definite height then goes to growing items, or to
 * justify-content (flex_col_free.rs). Returns the content height. */
pub(super) fn flex_col(
    node: &BoxNode,
    x: i32,
    y: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let s = &node.style;
    let mut laid: Vec<Laid> = Vec::new();
    let mut cy = 0i32;
    for (i, it) in node.children.iter().filter(|c| is_item(c)).enumerate() {
        let st = &it.style;
        cy = cy.saturating_add(if i > 0 { s.row_gap as i32 } else { 0 });
        let [mt, mr, mb, ml] = margins(st, w);
        let room = (w - ml - mr).max(0);
        let align = st.align_self.unwrap_or(s.align);
        let (bw, dx) = fit_across(it, room, align, s.rtl, depth);
        let pinned = Ctx { pin: Some(Pin { w: bw, h: None }), ..ctx };
        let start = frags.len();
        let h = layout_box(it, x + ml + dx, y + cy + mt, bw, frags, depth + 1, pinned);
        laid.push(Laid { node: it, ab: [start, frags.len()], h });
        cy = cy.saturating_add(mt + h + mb);
    }
    spread(&laid, cy, s, frags, ctx)
}
