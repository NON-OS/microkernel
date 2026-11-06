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

//! The file manager's Browse listing: every sort column, check box, row and
//! grid card is named where it is drawn, scrolled or not, and none overlap.

use nonos_toolkit::widgets::Rect;

use crate::fm::browse_hit::browse_at;
use crate::fm::grid_geom::{cell_slots, grid_scroll};
use crate::fm::layout::{content_w, content_x, FIRST_ROW_Y, FOOTER_H, GRID_CELL_H, GRID_CELL_W};
use crate::fm::layout::{GRID_TOP, ROW_H};
use crate::fm::list_cols::{check_w, cols, HEAD_H};
use crate::fm::pane::Pane;
use crate::fm::row_geom::{list_scroll, row_slots};
use crate::fm::target::Target;
use crate::fm_check::named_apart;

/// The listing of 60 entries scrolled to `scroll`, as `measure` lays it out.
pub fn pane(w: u32, h: u32, grid: bool, scroll: usize) -> Pane {
    let cols = (content_w(w) / GRID_CELL_W).max(1);
    let rows = (h.saturating_sub(GRID_TOP + FOOTER_H) / GRID_CELL_H).max(1) * cols;
    let list = (h.saturating_sub(FIRST_ROW_Y + FOOTER_H) / ROW_H).max(1);
    let view_rows = if grid { rows } else { list } as usize;
    let (row_top, row_h, len) = (FIRST_ROW_Y, ROW_H, 60);
    Pane { win_w: w, win_h: h, grid, len, scroll, row_top, row_h, view_rows, grid_cols: cols }
}

/// What the painters draw and where on screen; a row starts past its check box.
pub fn drawn(p: &Pane) -> Vec<(Target, Rect)> {
    let (at, mut out) = (|x, y, w, h| Rect { x, y, w, h }, Vec::new());
    for c in cell_slots(p).iter().filter(|_| p.grid) {
        let y = c.y - grid_scroll(p);
        out.push((Target::Cell(c.index), at(c.x, y, GRID_CELL_W, GRID_CELL_H)));
    }
    if p.grid {
        return out;
    }
    let c = cols(content_x(), content_w(p.win_w));
    for (i, h) in c.cols.iter().enumerate() {
        out.push((Target::Sort(i), at(h.x, p.row_top, h.w, HEAD_H)));
    }
    let row_w = content_x() + content_w(p.win_w) - c.tile_x;
    for s in row_slots(p) {
        let y = s.y - list_scroll(p);
        out.push((Target::Check(s.index), at(c.check_x, y, check_w(), ROW_H)));
        out.push((Target::Row(s.index), at(c.tile_x, y, row_w, ROW_H)));
    }
    out
}

#[test]
fn every_control_is_named_where_it_is_drawn_and_none_overlap() {
    for (w, h) in [(1100, 700), (1280, 560), (1600, 1000)] {
        for (grid, scroll) in [(false, 0), (false, 7), (true, 0), (true, 13)] {
            let p = pane(w, h, grid, scroll);
            let cs = drawn(&p);
            assert!(cs.len() > 4, "{w}x{h} grid {grid}");
            named_apart(&cs, |x, y| browse_at(&p, x, y), &alloc::format!("{w}x{h} at {scroll}"));
        }
    }
}
