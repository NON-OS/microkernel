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

//! Table: the tables, boxes and row counts the `cell_rect` proofs place
//! cells in, and how many rows fit whole in a box.

use nonos_toolkit::widgets::{Rect, Table};

use super::boxes::{boxed, boxes};
use super::table::{row_sets, COLS, ROWS};

/// Every column set with every row set, from the start, from row 3 and from
/// far down the list, each in its boxes and in boxes too small for it, with
/// its column count, row count and first row.
pub(crate) fn cases() -> Vec<(String, Table<'static>, Rect, (usize, usize, usize))> {
    let mut out = Vec::new();
    for (c, cols) in COLS {
        for (r, rows) in row_sets() {
            for first in [0, 3, 1_000_000] {
                let t = Table::new(cols).rows(rows).first(first);
                let mut ats = boxes(t.size()).to_vec();
                ats.extend([boxed(10, 10), boxed(40, 30), boxed(3, 200)]);
                let name = format!("{c}, {r}, from {first}");
                out.extend(
                    ats.into_iter()
                        .map(|at| (name.clone(), t, at, (cols.len(), rows.len(), first))),
                );
            }
        }
    }
    out
}

/// How many of `rows` given rows fit whole in `at`: the most whose table is
/// no taller than `at`, as `size` measures it.
pub(crate) fn fits(t: Table, at: Rect, rows: usize) -> usize {
    (1..=rows).take_while(|&k| t.rows(&ROWS[..k]).size().1 <= at.h).count()
}

/// The four corners and the middle of `r`.
pub(crate) fn marks(r: Rect) -> [(u32, u32); 5] {
    let (x1, y1) = (r.x + r.w - 1, r.y + r.h - 1);
    [(r.x, r.y), (x1, r.y), (r.x, y1), (x1, y1), (r.x + r.w / 2, r.y + r.h / 2)]
}
