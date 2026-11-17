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

extern crate alloc;

use alloc::vec::Vec;

use super::layout::{content_x, metrics};
use super::pane::Pane;

/// One drawn grid cell: the entry it shows and its top-left corner, in content
/// coordinates, which `Scrolled` moves up by `grid_scroll`.
pub struct CellSlot {
    pub index: usize,
    pub x: u32,
    pub y: u32,
}

/// The one layout pass for the icon grid. The first drawn cell is aligned to a
/// row boundary so wrapping stays stable while scrolling, and `paint_grid` and
/// `cell_at` both read the placement from here.
pub fn cell_slots(p: &Pane) -> Vec<CellSlot> {
    let cols = p.grid_cols.max(1) as usize;
    let start = p.scroll - (p.scroll % cols);
    let (left, m) = (content_x(), metrics());
    (start..p.len)
        .take(p.view_rows)
        .map(|index| CellSlot {
            index,
            x: left + (index % cols) as u32 * m.grid_cell_w,
            y: m.grid_top + (index / cols) as u32 * m.grid_cell_h,
        })
        .collect()
}

/// How far the grid is scrolled, in pixels: the rows above the first drawn.
pub fn grid_scroll(p: &Pane) -> u32 {
    (p.scroll / p.grid_cols.max(1) as usize) as u32 * metrics().grid_cell_h
}

/// Which entry the point `(x, y)` lands on, tested against exactly the cells
/// that were drawn. `y` is on the screen, and the scroll is added back the way
/// the painter took it off.
pub fn cell_at(p: &Pane, x: u32, y: u32) -> Option<usize> {
    let (y, m) = (y.saturating_add(grid_scroll(p)), metrics());
    let (w, h) = (m.grid_cell_w, m.grid_cell_h);
    cell_slots(p)
        .into_iter()
        .find(|c| x >= c.x && x < c.x + w && y >= c.y && y < c.y + h)
        .map(|c| c.index)
}
