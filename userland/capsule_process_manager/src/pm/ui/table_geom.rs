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

use alloc::vec;
use alloc::vec::Vec;

use nonos_toolkit::widgets::{Cell, Col as Column, Rect, Table};

use super::super::state::Sort;

pub use super::table_cols::{specs, Col, COLS_FULL, COLS_OVERVIEW};

// Everything measured from these columns (the row band and the visible slice)
// is a child module, so each file stays inside the 75-line cap while the
// painter and the hit test keep a single import path through here.
#[path = "table_span.rs"]
mod table_span;

pub use table_span::{head_h, row_h, row_y, visible_rows};

/// How many rows from `first` of `total` the painter hands a table `h` tall.
pub fn shown(first: usize, total: usize, h: u32) -> usize {
    visible_rows(h).min(total.saturating_sub(first))
}

/// What the shared table of `cols`, `w` across and `h` down, names at a point
/// from its top left, holding `n` rows from `first`: its own hit, on the rows
/// the painter hands it, so a click lands on the header or row drawn under it.
pub fn hit(
    cols: &[Column],
    (w, h): (u32, u32),
    (first, n): (usize, usize),
    x: i32,
    y: i32,
) -> Option<Cell> {
    let blank: Vec<&[&str]> = vec![&[]; n];
    Table::new(cols).rows(&blank).first(first).hit(Rect { x: 0, y: 0, w, h }, x, y)
}

// Only the columns refresh() can order by sort on a click; any other header
// is a no-op rather than a silently wrong reorder.
pub fn sort_for(col: Col) -> Option<Sort> {
    match col {
        Col::Name => Some(Sort::Name),
        Col::Pid => Some(Sort::Pid),
        Col::Cpu => Some(Sort::Cpu),
        Col::Mem => Some(Sort::Mem),
        Col::Ipc => Some(Sort::Ipc),
        Col::Sysc => Some(Sort::Sysc),
        _ => None,
    }
}

/// The index among `cols` of the column `sort` orders by, and whether it runs
/// smallest first, as refresh() orders name and pid. Cpu, memory and the
/// rates run largest first.
pub fn sorted(cols: &[Col], sort: Sort) -> Option<(usize, bool)> {
    let at = cols.iter().position(|c| sort_for(*c) == Some(sort))?;
    Some((at, matches!(sort, Sort::Name | Sort::Pid)))
}
