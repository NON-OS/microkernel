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

use crate::browser::css::GridTrack;

use super::super::abs_out_of_flow::out_of_flow;
use super::super::tree::BoxNode;
use super::intrinsic::contribution;

/* Each in-flow child's (min, max) contribution: its widths plus margins. */
fn items(n: &BoxNode, depth: u32) -> impl Iterator<Item = (i32, i32)> + '_ {
    n.children.iter().filter(|c| !out_of_flow(&c.style)).map(move |c| contribution(c, depth + 1))
}

/* Stacked children (block flow, a flex column): the widest of each. */
pub(in super::super) fn stack(n: &BoxNode, depth: u32) -> (i32, i32) {
    items(n, depth).fold((0, 0), |(a, b), (x, y)| (a.max(x), b.max(y)))
}

/* A flex row puts its items side by side with the column gap between
 * them; wrapping, it can be as narrow as its widest item. */
pub(in super::super) fn flex_row(n: &BoxNode, depth: u32) -> (i32, i32) {
    let (mut min, mut max, mut count) = (0i32, 0i32, 0i32);
    for (a, b) in items(n, depth) {
        min = if n.style.flex_wrap { min.max(a) } else { min.saturating_add(a) };
        max = max.saturating_add(b);
        count += 1;
    }
    let gaps = n.style.column_gap as i32 * (count - 1).max(0);
    (if n.style.flex_wrap { min } else { min.saturating_add(gaps) }, max.saturating_add(gaps))
}

/* A grid: items flow into its explicit columns in order, and each column
 * is as wide as its widest item, or its own px size. A grid without an
 * explicit column list is one column. */
pub(in super::super) fn grid(n: &BoxNode, depth: u32) -> (i32, i32) {
    let s = &n.style;
    let cols = if s.grid_auto.is_some() { 1 } else { (s.grid_col_n as usize).max(1) };
    let mut widths: Vec<(i32, i32)> = alloc::vec![(0, 0); cols];
    for (i, (a, b)) in items(n, depth).enumerate() {
        let w = &mut widths[i % cols];
        *w = (w.0.max(a), w.1.max(b));
    }
    for (i, w) in widths.iter_mut().enumerate() {
        if let (true, GridTrack::Px(p)) =
            (s.grid_col_n > 0, s.grid_cols[i.min(s.grid_cols.len() - 1)])
        {
            *w = (p as i32, p as i32);
        }
    }
    let gaps = s.column_gap as i32 * (cols as i32 - 1);
    let add =
        |(a, b): (i32, i32), (x, y): &(i32, i32)| (a.saturating_add(*x), b.saturating_add(*y));
    widths.iter().fold((gaps, gaps), add)
}
