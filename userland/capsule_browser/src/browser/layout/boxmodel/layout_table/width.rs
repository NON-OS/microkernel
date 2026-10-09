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

use crate::browser::css::Size;

use super::super::border_box_w::border_box_w;
use super::super::edges_x::edges_x;
use super::super::tree::BoxNode;
use super::columns::columns;
use super::grid::Grid;
use super::share::distribute;

/* A table's border-box width and its column widths. */
pub(super) struct Sized {
    pub w: i32,
    pub ws: Vec<i32>,
}

/* Size table `node` with grid `g` in `avail` px. Its width is its own
 * when set, else the sum of its columns' max-content widths up to
 * `avail` (CSS 2.1 17.5.2.2 shrink-to-fit), never below their
 * min-content sum. A column a cell gave a percentage takes that share
 * of the room the columns have when there is room: the share is the width
 * it prefers, so the columns still shrink toward their minima to fit. */
pub(super) fn size(node: &BoxNode, g: &Grid, avail: i32, depth: u32) -> Sized {
    let s = &node.style;
    let (sp, (el, er)) = (spacing(node), edges_x(s));
    let mut cols = columns(g, sp, depth + 1);
    let gaps = sp * (cols.len() as i32 + 1) + el + er;
    let lo = cols.iter().map(|c| c.min).sum::<i32>() + gaps;
    let hi = cols.iter().map(|c| c.max).sum::<i32>() + gaps;
    let own = border_box_w(s, avail);
    let w = if s.width == Size::Auto { hi.min(own) } else { own }.max(lo);
    let inner = (w - gaps).max(0);
    for c in cols.iter_mut().filter(|c| c.pct > 0) {
        let want = inner.saturating_mul(c.pct.min(100) as i32) / 100;
        (c.max, c.fixed) = (c.min.max(want), true);
    }
    Sized { w, ws: distribute(&cols, inner) }
}

/* The gap between cells and around them; none when borders collapse. */
pub(super) fn spacing(node: &BoxNode) -> i32 {
    if node.style.table.border_collapse {
        0
    } else {
        node.style.table.border_spacing as i32
    }
}
