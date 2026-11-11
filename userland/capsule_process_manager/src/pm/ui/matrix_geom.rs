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

use nonos_toolkit::widgets::{Cell, Col};

use crate::pm::security::sensitive as sens;

use super::table_geom;

// The ten sensitive authorities, grouped as sensitive.rs groups them. The label
// is the column header, abbreviated because MIN_UI_PX clamps the face at 17 px
// and "spawnbroker" is wider than any column here; auth_legend spells them out.
pub const MATRIX: [(u64, &[u8]); 10] = [
    (sens::ADMIN, b"ADM"),
    (sens::DEBUG, b"DBG"),
    (sens::DRIVER, b"DRV"),
    (sens::MMIO, b"MMIO"),
    (sens::IRQ, b"IRQ"),
    (sens::DMA, b"DMA"),
    (sens::PIO, b"PIO"),
    (sens::SPAWN_BROKER, b"SPWB"),
    (sens::SPAWN_WINDOW, b"SPWW"),
    (sens::PROC_CTL, b"PCTL"),
];

pub const PAD_X: u32 = 14;
pub const NAME_W: u32 = 232;
pub const CELL_DOT: u32 = 12;
pub const LEGEND_H: u32 = 48;

/// The shared table's columns: PROCESS, then each authority's short name.
pub fn cols() -> Vec<Col<'static>> {
    let short = |(_, label): &(u64, &'static [u8])| core::str::from_utf8(label).unwrap_or("");
    let names = MATRIX.iter().map(|c| Col::new(short(c)));
    core::iter::once(Col::new("PROCESS").fixed(NAME_W)).chain(names).collect()
}

/// The grid's height: the pane less the legend strip under it.
pub fn grid_h(pane_h: u32) -> u32 {
    pane_h.saturating_sub(LEGEND_H)
}

pub fn visible_rows(pane_h: u32) -> usize {
    (grid_h(pane_h).saturating_sub(table_geom::head_h()) / table_geom::row_h().max(1)) as usize
}

/// How many rows from `first` of `total` the painter hands the grid.
pub fn shown(first: usize, total: usize, pane_h: u32) -> usize {
    visible_rows(pane_h).min(total.saturating_sub(first))
}

// Pane-local, as the table's hit is. The grid stops above the legend, so the
// strip under it is drawn in the same pane but is never a row.
pub fn hit(table_w: u32, pane_h: u32, first: usize, total: usize, x: i32, y: i32) -> Option<Cell> {
    let rows = (first, shown(first, total, pane_h));
    table_geom::hit(&cols(), (table_w, grid_h(pane_h)), rows, x, y)
}
