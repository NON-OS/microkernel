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

//! The body's rows: colours after every attribute, wide characters across
//! two cells, and the selection and search match shaded.

use nonos_app_skeleton::PaintBuffer;
use nonos_vt::Line;

use super::area::{Frame, Shade};
use super::cell::text;
use super::cursor::draw_cursor;
use super::deco::decorations;
use super::fill::background;
use super::rows::Rows;

pub fn draw_vt(f: &Frame, fb: &mut PaintBuffer, rows: Rows, shade: Shade) {
    let top = rows.first(f.vt);
    for i in 0.. {
        let y = crate::layout::row_top(i as u32, f.area.y, f.m.lh);
        if y + f.m.lh > f.area.max_y {
            break;
        }
        let Some((abs, line)) = rows.get(f.vt, top, i) else { break };
        draw_row(f, fb, line, abs, y, shade);
        draw_cursor(f, fb, rows, abs, y);
    }
}

fn draw_row(f: &Frame, fb: &mut PaintBuffer, line: &Line, abs: u64, y: u32, shade: Shade) {
    let m = f.m;
    for col in 0..f.vt.cols() {
        let x = f.area.x + col as u32 * m.adv;
        if x + m.adv > f.area.max_x {
            break;
        }
        let cell = line.cell(col);
        let (fg, bg) = f.vt.cell_colors(&cell);
        if let Some(bg) = background(f, &cell, bg, abs, col, shade) {
            fb.fill_rect(x, y, m.adv, m.lh, bg);
        }
        /*
         * The right half of a wide character carries only its background;
         * the head drew the glyph across both cells.
         */
        if !cell.is_tail() {
            let w = if cell.is_wide() { 2 * m.adv } else { m.adv };
            text(fb, line, &cell, x, y, fg, m);
            decorations(fb, &cell, x, y, w, fg, m);
        }
    }
}
