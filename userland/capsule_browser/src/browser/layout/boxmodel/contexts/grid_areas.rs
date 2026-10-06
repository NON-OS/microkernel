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

use crate::browser::css::Computed;

use super::super::tree::BoxNode;
use super::grid_auto::{place, Req};
use super::grid_occupy::{Area, MAX_SPAN};

/* One axis of a placement against `n` explicit tracks: (first track when
 * definite, span). A negative line counts back from the explicit grid's
 * last line; lines given both ways round span between them. */
fn resolve(lines: [i16; 2], span: u16, n: usize) -> (Option<usize>, usize) {
    let idx = |l: i16| match l {
        0 => None,
        l if l > 0 => Some(l as i32 - 1),
        l => Some((n.min(MAX_SPAN) as i32 + 1 + l as i32).max(0)),
    };
    let span = (span as usize).clamp(1, MAX_SPAN);
    match (idx(lines[0]), idx(lines[1])) {
        (Some(a), Some(b)) if a != b => (Some(a.min(b) as usize), (a - b).unsigned_abs() as usize),
        (Some(a), Some(_)) => (Some(a as usize), 1),
        (Some(a), None) => (Some(a as usize), span),
        (None, Some(b)) => (Some((b - span as i32).max(0) as usize), span),
        (None, None) => (None, span),
    }
}

/* Place the items of a grid with `n_cols` explicit columns: their areas as
 * (row, column), and the column and row counts, explicit and implicit,
 * each at least one. Column flow places down the columns instead. */
pub(in super::super) fn areas(
    items: &[&BoxNode],
    n_cols: usize,
    s: &Computed,
) -> (Vec<Area>, usize, usize) {
    let (n_rows, fc) = (s.grid_row_n as usize, s.grid_flow_col);
    let reqs: Vec<Req> = items
        .iter()
        .map(|it| {
            let p = it.grid_place;
            let col = p.map_or((None, 1), |p| resolve(p.col, p.col_span, n_cols));
            let row = p.map_or((None, 1), |p| resolve(p.row, p.row_span, n_rows));
            if fc {
                [col, row]
            } else {
                [row, col]
            }
        })
        .collect();
    let minor = if fc { n_rows } else { n_cols };
    let (placed, m, grown) = place(&reqs, minor, s.grid_dense);
    if fc {
        let swapped =
            placed.into_iter().map(|a| Area { r: a.c, c: a.r, rs: a.cs, cs: a.rs }).collect();
        (swapped, grown.max(n_cols).max(1), m.max(1))
    } else {
        (placed, m.max(1), grown.max(n_rows).max(1))
    }
}
