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

use super::super::tree::BoxNode;
use super::grid::{Grid, Slot, MAX_COLS};

/* The rows among the children of `parent` (the table when `top`, else
 * a row group): a row box, a run of bare cells as one anonymous row, a
 * row group's own rows, or at the top a display:table-caption box. Any
 * other child is not part of the table model here and is not laid out
 * (CSS 2.1 17.2.1 would wrap it in an anonymous cell; the stray boxes
 * met are whitespace and markup a parser leaked as text). */
pub(super) fn rows_of<'a>(parent: &'a BoxNode, g: &mut Grid<'a>, busy: &mut Vec<u32>, top: bool) {
    let kids = &parent.children;
    let mut i = 0;
    while i < kids.len() {
        let c = &kids[i];
        let run = kids[i..].iter().take_while(|k| k.style.is_table_cell).count();
        if run > 0 {
            row(None, &kids[i..i + run], g, busy);
            i += run;
            continue;
        }
        if c.style.is_table_row {
            row(Some(c), &c.children, g, busy);
        } else if top && c.children.iter().any(|k| k.style.is_table_row || k.style.is_table_cell) {
            rows_of(c, g, busy, false);
        } else if top && c.style.table.caption {
            g.captions.push(c);
        }
        i += 1;
    }
}

/* Row `node` with `cells`, each placed in the first free column. */
fn row<'a>(node: Option<&'a BoxNode>, cells: &'a [BoxNode], g: &mut Grid<'a>, busy: &mut Vec<u32>) {
    let r = g.rows.len() as u32;
    g.rows.push(node);
    let mut c = 0usize;
    for cell in cells.iter().filter(|k| k.style.is_table_cell) {
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
