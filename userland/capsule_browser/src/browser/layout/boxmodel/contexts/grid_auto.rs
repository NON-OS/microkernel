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

use super::grid_cursor::next_free;
use super::grid_locked::place_locked;
use super::grid_occupy::{Area, Occupy, MAX_COLS, MAX_ROWS};

/* One item's request on the (grown, filled) axes: start, when definite,
 * and span. In row flow the grown axis is rows; column flow swaps them. */
pub(in super::super) type Req = [(Option<usize>, usize); 2];

/* Cells a dense placement may test before later items fall back to the
 * forward-only cursor, so a hostile dense grid still places in time. */
const SCAN_BUDGET: usize = 1 << 20;

/* Place every item, as CSS grid auto-placement does (Grid 8.5): first the
 * items definite on both axes, then those locked to a grown-axis track,
 * then the rest in order behind a cursor that only moves forward (sparse)
 * or restarts for each item (dense). `n` is the explicit track count of
 * the filled axis; the filled axis widens to fit what is asked of it, up
 * to MAX_COLS. Returns the areas, the filled-axis count, the grown count. */
pub(in super::super) fn place(reqs: &[Req], n: usize, dense: bool) -> (Vec<Area>, usize, usize) {
    let need = |(s, sp): (Option<usize>, usize)| s.map_or(sp, |s| s + sp);
    let m = reqs.iter().map(|q| need(q[1])).fold(n.max(1), usize::max).min(MAX_COLS);
    let fit = |(s, sp): (Option<usize>, usize)| {
        let sp = sp.clamp(1, m);
        (s.map(|s| s.min(m - sp)), sp)
    };
    let reqs: Vec<Req> = reqs.iter().map(|q| [q[0], fit(q[1])]).collect();
    let mut occ = Occupy::new();
    let mut out: Vec<Option<Area>> = alloc::vec![None; reqs.len()];
    let area =
        |r: usize, rs: usize, c: usize, cs: usize| Area { r: r.min(MAX_ROWS - 1), rs, c, cs };
    for (i, q) in reqs.iter().enumerate() {
        if let ((Some(r), rs), (Some(c), cs)) = (q[0], q[1]) {
            out[i] = Some(area(r, rs, c, cs));
            occ.mark(area(r, rs, c, cs));
        }
    }
    place_locked(&reqs, &mut occ, &mut out, m, dense);
    let (mut cursor, mut budget) = ((0usize, 0usize), SCAN_BUDGET);
    for i in 0..reqs.len() {
        if out[i].is_none() {
            let (q, dense) = (reqs[i], dense && budget > 0);
            let a = next_free(&occ, &mut cursor, [q[0].1, q[1].1], q[1].0, m, dense, &mut budget);
            occ.mark(a);
            out[i] = Some(a);
        }
    }
    (out.into_iter().map(Option::unwrap_or_default).collect(), m, occ.rows())
}
