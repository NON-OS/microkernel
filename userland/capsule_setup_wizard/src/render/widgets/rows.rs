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

use crate::render::ink::{body_line, draw_text};
use crate::render::layout::layout;
use crate::render::paint::fill_rect;
use crate::render::theme::{ACCENT, FG, HINT, ROW_BORDER, ROW_SEL_BG};

/// A choice list on thin rules, the selected row lit: a soft fill, a cyan
/// bar at its edge, its text in full white. Rows are the layout's row height
/// and list width, so the list is a whole number of units tall; returns its
/// bottom edge, where the closing rule starts the gap that follows it.
pub fn list(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    items: &[&[u8]],
    sel: usize,
) -> u32 {
    let l = layout(w, h);
    let (rw, rh) = (l.list_w, l.row_h);
    let hair = l.scale.px(1).max(1);
    let text_dy = rh.saturating_sub(body_line(w, h)) / 2;
    let mut yy = y;
    for (i, item) in items.iter().enumerate() {
        fill_rect(buf, spx, w, h, x, yy, rw, hair, ROW_BORDER);
        if i == sel {
            fill_rect(buf, spx, w, h, x, yy + hair, rw, rh - hair, ROW_SEL_BG);
            fill_rect(buf, spx, w, h, x, yy + rh / 4, l.scale.px(2), rh / 2, ACCENT);
        }
        let color = if i == sel { FG } else { HINT };
        draw_text(buf, spx, w, h, x + 2 * l.unit, yy + text_dy, item, color);
        yy += rh;
    }
    fill_rect(buf, spx, w, h, x, yy, rw, hair, ROW_BORDER);
    yy
}
