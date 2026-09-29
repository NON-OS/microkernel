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

use crate::browser::css::{Size, WhiteSpace};

use super::super::content_width::content_width;
use super::super::edges_x::edges_x;
use super::super::min_content_width::min_content_width;
use super::super::tree::{BoxKind, BoxNode};
use super::columns::columns;
use super::grid::grid;

const MAX_DEPTH: u32 = 400;
pub(super) const CAP: i32 = 8192;

/* The (min-content, max-content) border-box widths of `node`. A table
 * sums its columns and spacing; a block stacks its children, so a table
 * inside it counts as a table; anything else is measured by the shared
 * intrinsic walks. white-space: nowrap makes the minimum the maximum. */
pub(in super::super) fn intrinsic(node: &BoxNode, depth: u32) -> (i32, i32) {
    if depth > MAX_DEPTH {
        return (0, 0);
    }
    let s = &node.style;
    let block = matches!(node.kind, BoxKind::Block);
    let (mn, mx) = if s.is_table {
        table(node, depth)
    } else if let Some(px) = s.width.definite_px() {
        (px.clamp(0, CAP), px.clamp(0, CAP))
    } else if block && node.children.iter().any(|c| c.kind.block_level()) {
        super::stack::stacked(node, depth)
    } else {
        (min_content_width(node, depth), content_width(node, depth))
    };
    let mn = if s.white_space == WhiteSpace::Nowrap { mn.max(mx) } else { mn };
    (mn, mx.max(mn))
}

/* A table: its columns' widths, the spacing around them and its own
 * edges, at least its fixed width and its widest caption. */
fn table(node: &BoxNode, depth: u32) -> (i32, i32) {
    let s = &node.style;
    let g = grid(node);
    let (el, er) = edges_x(s);
    let sp = super::width::spacing(node);
    let cols = columns(&g, sp, depth + 1);
    let extra = sp * (cols.len() as i32 + 1) + el + er;
    let mut mn = cols.iter().map(|c| c.min).sum::<i32>() + extra;
    let mut mx = cols.iter().map(|c| c.max).sum::<i32>() + extra;
    for c in &g.captions {
        mn = mn.max(intrinsic(c, depth + 1).0);
    }
    if let Size::Px(px) = s.width {
        let w = px as i32 + if s.border_box { 0 } else { el + er };
        (mn, mx) = (mn.max(w), mn.max(w));
    }
    (mn.clamp(0, CAP), mx.max(mn).clamp(0, CAP))
}
