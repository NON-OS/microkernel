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

//! The cursor, in the shape the program set: block, underline or bar.

use nonos_app_skeleton::PaintBuffer;
use nonos_vt::CursorShape;

use super::area::{Frame, OPAQUE};
use super::cell::glyph;
use super::rows::Rows;

/// Draw the cursor if it is on the row at `abs`, drawn at `y`. The shell
/// view draws its own cursor in the prompt instead.
pub fn draw_cursor(f: &Frame, fb: &mut PaintBuffer, rows: Rows, abs: u64, y: u32) {
    let c = f.vt.cursor();
    if !matches!(rows, Rows::Screen) || !c.visible || f.vt.abs_of_row(c.y) != abs {
        return;
    }
    let (m, x) = (f.m, f.area.x + c.x as u32 * f.m.adv);
    if x + m.adv > f.area.max_x {
        return;
    }
    let colour = OPAQUE | f.vt.palette().cursor;
    match c.shape {
        CursorShape::Block => {
            fb.fill_rect(x, y, m.adv, m.lh, colour);
            let under = f.vt.visible_line(c.y).cell(c.x);
            glyph(fb, x, y, under.ch, OPAQUE | f.t.bg, m.px);
        }
        CursorShape::Underline => fb.fill_rect(x, y + m.lh.saturating_sub(2), m.adv, 2, colour),
        CursorShape::Bar => fb.fill_rect(x, y, 2, m.lh, colour),
    }
}
