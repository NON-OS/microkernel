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

use super::grid_auto::Req;
use super::grid_occupy::{Area, Occupy, MAX_COLS, MAX_ROWS};

/* An item locked to row `a.r`: the first free place from column `a.c`
 * on, never before it (Grid 8.5 step 2), past the columns there are if it
 * must be, which makes implicit ones. Only when even the widest grid has
 * no room does it overlap, at the cursor or the last place its span fits. */
fn fill_row(occ: &Occupy, a: Area) -> Area {
    let last = MAX_COLS.saturating_sub(a.cs);
    let at = |c: usize| Area { c, ..a };
    (a.c.min(last)..=last).map(at).find(|&t| occ.fits(t)).unwrap_or(at(a.c.min(last)))
}

/* Place the items locked to a row (a definite row, an auto column) in
 * order, into `out`: each from where the last item in its row ended
 * (sparse), or from the row's start (dense). Returns the column count `m`
 * grown to hold them. */
pub(in super::super) fn place_locked(
    reqs: &[Req],
    occ: &mut Occupy,
    out: &mut [Option<Area>],
    m: usize,
    dense: bool,
) -> usize {
    let (mut row_cursor, mut m): (Vec<(usize, usize)>, usize) = (Vec::new(), m);
    for (i, q) in reqs.iter().enumerate() {
        if let ((Some(r), rs), (None, cs)) = (q[0], q[1]) {
            let r = r.min(MAX_ROWS - 1);
            let from =
                if dense { 0 } else { row_cursor.iter().find(|p| p.0 == r).map_or(0, |p| p.1) };
            let a = fill_row(occ, Area { r, rs, c: from, cs });
            row_cursor.retain(|p| p.0 != r);
            row_cursor.push((r, a.c + cs));
            m = m.max(a.c + cs);
            occ.mark(a);
            out[i] = Some(a);
        }
    }
    m
}
