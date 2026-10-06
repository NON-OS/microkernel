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
use super::grid::Grid;
use super::row::{in_table_cell, row};

/* The rows among the children of `parent` (the table when `top`, else
 * a row group): a row box, a run of bare cells as one anonymous row, a
 * row group's own rows, or at the top a display:table-caption box. Any
 * other child is not part of the table model here and is not laid out
 * (CSS 2.1 17.2.1 would wrap it in an anonymous cell; the stray boxes
 * met are whitespace and markup a parser leaked as text), nor is an
 * absolutely positioned one: it leaves the table (CSS 2.1 9.7). */
pub(super) fn rows_of<'a>(parent: &'a BoxNode, g: &mut Grid<'a>, busy: &mut Vec<u32>, top: bool) {
    let kids = &parent.children;
    let mut i = 0;
    while i < kids.len() {
        let c = &kids[i];
        if out_of_flow(&c.style) {
            i += 1;
            continue;
        }
        let run = kids[i..].iter().take_while(|k| in_table_cell(k)).count();
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
