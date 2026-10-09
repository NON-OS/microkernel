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

//! Breaking one logical line at a new width, keeping wide characters whole
//! and noting where the cursor lands.

use alloc::vec::Vec;

use super::placed::Placed;
use crate::cell::{attr, Cell};
use crate::line::Line;

/// Break one logical line of `cells` (each tagged with the physical line it
/// came from, for its marks) into lines `cols` wide.
pub(super) fn rewrap(
    phys: &[Line],
    cells: &[(usize, Cell)],
    cols: usize,
    cursor: Option<usize>,
    out: &mut Vec<Line>,
    placed: &mut Option<Placed>,
) {
    let mut line = Line::new(0, Cell::BLANK);
    let mut col: usize = 0;
    for (i, &(src, cell)) in cells.iter().enumerate() {
        if cell.is_tail() {
            if cursor == Some(i) && placed.is_none() {
                *placed =
                    Some(Placed { line: out.len(), col: col.saturating_sub(1), pending: false });
            }
            continue;
        }
        let w = if cell.is_wide() && cols >= 2 { 2 } else { 1 };
        if col + w > cols {
            line.cells.resize(cols, Cell::BLANK);
            line.wrapped = true;
            out.push(core::mem::replace(&mut line, Line::new(0, Cell::BLANK)));
            col = 0;
        }
        if cursor == Some(i) {
            *placed = Some(Placed { line: out.len(), col, pending: false });
        }
        let head = if w == 2 { cell } else { Cell { attr: cell.attr & !attr::WIDE, ..cell } };
        line.push_from(&phys[src], head);
        if w == 2 {
            let tail = (cell.attr & !attr::WIDE) | attr::WIDE_TAIL;
            line.cells.push(Cell { ch: ' ', attr: tail, mark: 0, ..cell });
        }
        col += w;
    }
    if cursor == Some(cells.len()) {
        *placed = Some(if col >= cols {
            Placed { line: out.len(), col: cols - 1, pending: true }
        } else {
            Placed { line: out.len(), col, pending: false }
        });
    }
    line.cells.resize(cols, Cell::BLANK);
    out.push(line);
}
