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
use super::contexts::flex_item::{flex_item, FlexItem};
use super::contexts::flex_line::{lay_line, Line};
use super::contexts::flex_main::main_offsets;
use super::contexts::flex_resolve::resolve;
use super::contexts::flex_wrap::{auto_margins, line_end};
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::tree::BoxNode;

/* A flex row at (x, y), `w` wide. Items break into lines when the box
 * wraps; on each line their sizes flex (grow into free space, shrink out
 * of missing space, within their min and max), the leftover goes to auto
 * margins or justify-content, and each item is laid out once at its final
 * width, which is pinned so it is not resolved again. A line is as tall as
 * its tallest item, or, when the box is single-line with a definite
 * height, that height. Returns the height of all lines. */
pub(super) fn flex_row(
    node: &BoxNode,
    x: i32,
    y: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let s = &node.style;
    let mut items: Vec<FlexItem> =
        node.children.iter().filter(|c| is_item(c)).map(|c| flex_item(c, w, depth + 1)).collect();
    let (gap, wrap) = (s.column_gap as i32, s.flex_wrap);
    let definite = ctx.cb.h.filter(|_| !wrap);
    let mut cy = 0i32;
    let mut a = 0usize;
    while a < items.len() {
        let b = line_end(&items, a, w, gap, wrap);
        if a > 0 {
            cy = cy.saturating_add(s.row_gap as i32);
        }
        let line = &mut items[a..b];
        let gaps = gap.saturating_mul(line.len() as i32 - 1);
        resolve(line, w.saturating_sub(gaps));
        let used = line.iter().fold(gaps, |u, it| u.saturating_add(it.outer()));
        let autos = auto_margins(line);
        let offsets =
            main_offsets(w.saturating_sub(used), line.len() as i32, autos, gap, s.justify);
        let l = Line { r: [x, y.saturating_add(cy), w], offsets, cross: definite, style: s };
        cy = cy.saturating_add(lay_line(line, &l, frags, depth, ctx));
        a = b;
    }
    cy
}
