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

use super::super::abs_out_of_flow::out_of_flow;
use super::super::tree::BoxNode;
use super::grid::{Grid, Slot, MAX_COLS};

/* Row `node` with `cells`, each placed in the first free column. */
pub(super) fn row<'a>(
    node: Option<&'a BoxNode>,
    cells: &'a [BoxNode],
    g: &mut Grid<'a>,
    busy: &mut Vec<u32>,
) {
    let r = g.rows.len() as u32;
    g.rows.push(node);
    let mut c = 0usize;
    for cell in cells.iter().filter(|k| in_table_cell(k)) {
        while busy.get(c).is_some_and(|&b| b > r) {
            c += 1;
        }
        if c >= MAX_COLS {
            break;
        }
        let cs = (cell.style.table.col_span as usize).clamp(1, MAX_COLS - c);
        let rs = u32::from(cell.style.table.row_span.max(1));
        if busy.len() < c + cs {
            busy.resize(c + cs, 0);
        }
        busy[c..c + cs].iter_mut().for_each(|b| *b = r.saturating_add(rs));
        g.slots.push(Slot { cell, row: r, col: c as u32, cs: cs as u32, rs });
        c += cs;
    }
}

/* A cell that takes part in the table: an absolutely positioned one does not. */
pub(super) fn in_table_cell(k: &BoxNode) -> bool {
    k.style.is_table_cell && !out_of_flow(&k.style)
}
