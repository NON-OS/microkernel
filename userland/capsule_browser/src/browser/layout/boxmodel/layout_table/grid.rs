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

/* Columns a table keeps; a cell starting past them is not laid out
 * (the HTML table model caps a colspan at 1,000 as well). */
pub(super) const MAX_COLS: usize = 1000;

/* One cell in the table grid: its first row and column and its spans. */
pub(super) struct Slot<'a> {
    pub cell: &'a BoxNode,
    pub row: u32,
    pub col: u32,
    pub cs: u32,
    pub rs: u32,
}

/* A table's rows (the row box, None for cells with no row around them),
 * its cells in row order, its column count and its caption boxes. */
#[derive(Default)]
pub(super) struct Grid<'a> {
    pub rows: Vec<Option<&'a BoxNode>>,
    pub slots: Vec<Slot<'a>>,
    pub ncols: usize,
    pub captions: Vec<&'a BoxNode>,
}

/* Build the grid of `table` (HTML's table model, 4.9.12): rows come
 * from the table and its row groups in order, and each cell takes the
 * first column no rowspan above it still holds. Cells directly in the
 * table or a group make an anonymous row; other blocks are captions. */
pub(super) fn grid(table: &BoxNode) -> Grid<'_> {
    let mut g = Grid::default();
    /* busy[c]: the first row at which column c is free again. */
    let mut busy: Vec<u32> = Vec::new();
    super::rows::rows_of(table, &mut g, &mut busy, true);
    let n = g.rows.len() as u32;
    for s in g.slots.iter_mut() {
        s.rs = s.rs.clamp(1, n - s.row);
    }
    g.ncols = busy.len();
    g
}
