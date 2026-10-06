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

use super::grid_occupy::{Area, Occupy, MAX_ROWS};

/* The next place for an item not locked to a row, behind `cursor` (row,
 * column). With a definite column the cursor drops to the first row where
 * that column is free (a column left of the cursor starts a new row);
 * otherwise it walks along rows to the first free place the span fits. A
 * dense item starts over from the first row. Each cell tested costs one
 * from `budget`. Rows end at MAX_ROWS, where anything left stacks. */
pub(in super::super) fn next_free(
    occ: &Occupy,
    cursor: &mut (usize, usize),
    spans: [usize; 2],
    col: Option<usize>,
    m: usize,
    dense: bool,
    budget: &mut usize,
) -> Area {
    let [rs, cs] = spans;
    if dense {
        *cursor = (0, if col.is_some() { cursor.1 } else { 0 });
    }
    if let Some(c) = col {
        if c < cursor.1 && !dense {
            cursor.0 += 1;
        }
        cursor.1 = c;
        while cursor.0 < MAX_ROWS - 1 && !occ.fits(Area { r: cursor.0, rs, c, cs }) {
            (cursor.0, *budget) = (cursor.0 + 1, budget.saturating_sub(1));
        }
        return Area { r: cursor.0.min(MAX_ROWS - 1), rs, c, cs };
    }
    loop {
        if cursor.1 + cs > m {
            *cursor = (cursor.0 + 1, 0);
        }
        if cursor.0 >= MAX_ROWS - 1 {
            return Area { r: MAX_ROWS - 1, rs, c: cursor.1.min(m.saturating_sub(cs)), cs };
        }
        let a = Area { r: cursor.0, rs, c: cursor.1, cs };
        *budget = budget.saturating_sub(1);
        if occ.fits(a) {
            cursor.1 += cs;
            return a;
        }
        cursor.1 += 1;
    }
}
