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

//! Table: `hit` names each cell where `cell_rect` says it is, for every cell
//! of every column set and row count, scrolled or not, in every box at every
//! scale. A rect lies in the table, and a cell past the last column, before
//! `first` or below the rows that fit has none.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Cell, Rect, Table};

use super::boxes::within;
use super::serial::{serial, SCALES};
use super::table_place_set::{cases, fits, marks};

/// What is wrong with the cells of `t`, with `cols` columns and `rows` rows
/// from `first`, in `at`.
fn cells(t: Table, at: Rect, (cols, rows, first): (usize, usize, usize)) -> Vec<String> {
    let mut bad = Vec::new();
    let fit = fits(t, at, rows);
    let hits = |r: Rect, want: Cell| {
        marks(r).iter().all(|&(x, y)| t.hit(at, x as i32, y as i32) == Some(want))
    };
    for k in 0..rows + 2 {
        for col in 0..cols + 1 {
            let row = first + k;
            match (t.cell_rect(at, row, col), k < fit && col < cols) {
                (None, false) => {}
                (Some(r), true)
                    if within(r, at) && (r.w == 0 || hits(r, Cell::Row { row, col })) => {}
                (got, _) => bad.push(format!("row {row} col {col}: {got:?}")),
            }
        }
    }
    if first > 0 && t.cell_rect(at, first - 1, 0).is_some() {
        bad.push(format!("row {} before first has a rect", first - 1));
    }
    bad
}

#[test]
fn table_cell_rect_is_where_hit_names_each_cell() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, t, at, n) in cases() {
            fails.extend(cells(t, at, n).into_iter().map(|e| format!("{name} {at:?} {q}/4: {e}")));
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
