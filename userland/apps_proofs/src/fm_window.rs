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

//! The file manager's Browse listing as `measure` lays it out at the display
//! scale in force, and the boxes its painters draw.

use alloc::vec::Vec;

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::Rect;

use crate::fm::grid_geom::{cell_slots, grid_scroll};
use crate::fm::layout::{content_w, content_x, metrics};
use crate::fm::list_cols::{check_w, cols, head_h};
use crate::fm::pane::Pane;
use crate::fm::row_geom::{list_scroll, row_slots};
use crate::fm::target::Target;

/// `w` by `h` logical pixels at the display scale in force.
pub fn dims(w: u32, h: u32) -> (u32, u32) {
    (scale().px(w), scale().px(h))
}

/// The listing of 60 entries scrolled to `scroll` in a `w` by `h` drawing
/// pixel window, as `measure` lays it out.
pub fn pane(w: u32, h: u32, grid: bool, scroll: usize) -> Pane {
    let m = metrics();
    let cols = (content_w(w) / m.grid_cell_w).max(1);
    let rows = (h.saturating_sub(m.grid_top + m.footer_h) / m.grid_cell_h).max(1) * cols;
    let list = (h.saturating_sub(m.first_row_y + m.footer_h) / m.row_h).max(1);
    let view_rows = if grid { rows } else { list } as usize;
    let (row_top, row_h, len) = (m.first_row_y, m.row_h, 60);
    Pane { win_w: w, win_h: h, grid, len, scroll, row_top, row_h, view_rows, grid_cols: cols }
}

/// What the painters draw and where on screen; a row starts past its check box.
pub fn drawn(p: &Pane) -> Vec<(Target, Rect)> {
    let (at, mut out, m) = (|x, y, w, h| Rect { x, y, w, h }, Vec::new(), metrics());
    for c in cell_slots(p).iter().filter(|_| p.grid) {
        let y = c.y - grid_scroll(p);
        out.push((Target::Cell(c.index), at(c.x, y, m.grid_cell_w, m.grid_cell_h)));
    }
    if p.grid {
        return out;
    }
    let c = cols(content_x(), content_w(p.win_w));
    for (i, h) in c.cols.iter().enumerate() {
        out.push((Target::Sort(i), at(h.x, p.row_top, h.w, head_h())));
    }
    let row_w = content_x() + content_w(p.win_w) - c.tile_x;
    for s in row_slots(p) {
        let y = s.y - list_scroll(p);
        out.push((Target::Check(s.index), at(c.check_x, y, check_w(), m.row_h)));
        out.push((Target::Row(s.index), at(c.tile_x, y, row_w, m.row_h)));
    }
    out
}
