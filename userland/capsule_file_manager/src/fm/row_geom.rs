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

//! The detail list's rows, laid out over a `Pane` in content coordinates.

extern crate alloc;

use alloc::vec::Vec;

use super::layout::metrics;
use super::list_cols::head_h;
use super::pane::Pane;

/// One drawn list line: the entry it shows and its y in content coordinates,
/// which `Scrolled` moves up by `list_scroll` on the way to the screen.
pub struct RowSlot {
    pub index: usize,
    pub y: u32,
}

/// Top of the first row. The column header strip owns the band above it, so the
/// listing starts one strip below the origin `layout_measure::measure` stashed.
pub fn list_top(p: &Pane) -> u32 {
    p.row_top + head_h()
}

/// How many entries the active view actually shows. `layout_measure::measure` sized
/// `view_rows` for a list with no header strip, so the list count is re-derived
/// here against the same window height rather than letting the last row draw
/// over the footer. Scroll clamping reads this, so the cursor cannot walk off
/// the bottom of what was drawn.
pub fn visible_rows(p: &Pane) -> usize {
    if p.grid {
        return p.view_rows.max(1);
    }
    let avail = p.win_h.saturating_sub(list_top(p) + metrics().footer_h);
    ((avail / p.row_h.max(1)) as usize).min(p.view_rows).max(1)
}

/// How far the list is scrolled, in pixels: the rows above the first drawn.
pub fn list_scroll(p: &Pane) -> u32 {
    (p.scroll as u32).saturating_mul(p.row_h)
}

/// The one layout pass for the detail list, in the shape `sidebar_rows` uses:
/// `paint_rows` draws exactly these slots and `row_at` tests against them, so a
/// click can never land on a row that was not drawn.
pub fn row_slots(p: &Pane) -> Vec<RowSlot> {
    let top = list_top(p);
    let y = |index: usize| top.saturating_add((index as u32).saturating_mul(p.row_h));
    (p.scroll..p.len).take(visible_rows(p)).map(|index| RowSlot { index, y: y(index) }).collect()
}

/// Which entry the point `y` lands on; `None` above the first row, below the
/// last drawn one, or past the end of the listing. `y` is on the screen, and
/// the scroll is added back the way the painter took it off.
pub fn row_at(p: &Pane, y: u32) -> Option<usize> {
    let (h, y) = (p.row_h.max(1), y.saturating_add(list_scroll(p)));
    row_slots(p).into_iter().find(|s| y >= s.y && y < s.y + h).map(|s| s.index)
}
