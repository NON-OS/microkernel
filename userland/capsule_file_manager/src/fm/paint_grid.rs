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

//! Icon-grid view: each entry on its own rounded panel with a large filetype
//! glyph and a measured, truncated label. Cells are placed by
//! `grid_geom::cell_slots`, the same list `cell_at` hit-tests against.

use nonos_app_skeleton::PaintBuffer;

use super::file_color::color;
use super::file_kind::kind_of;
use super::grid_geom::cell_slots;
use super::icon;
use super::layout::{GRID_CELL_H, GRID_CELL_W, GRID_ICON};
use super::measure_text::{truncate_to_width, width_of};
use super::state::State;
use super::theme::{CY, INK, INK2, LINE, PANEL, RAISE};

// The panel is inset inside its cell so neighbouring tiles never touch, and the
// label sits in the strip left below the glyph.
const INSET_X: u32 = 6;
const INSET_Y: u32 = 3;
const LABEL_PX: f32 = 14.0;

pub fn paint_grid(state: &State, fb: &mut PaintBuffer) {
    let pw = GRID_CELL_W - INSET_X * 2;
    let ph = GRID_CELL_H - INSET_Y * 2;
    for cell in cell_slots(state) {
        let entry = &state.entries[cell.index];
        let lit = cell.index == state.cursor
            || state.selected.iter().any(|path| path == &entry.full_path);
        let (fill, border) = if lit { (RAISE, CY) } else { (PANEL, LINE) };
        let (px, py) = (cell.x + INSET_X, cell.y + INSET_Y);
        fb.panel(px, py, pw, ph, 12, fill, border);

        let tint = color(kind_of(entry));
        let gx = px + (pw - GRID_ICON) / 2;
        let gy = py + 12;
        if entry.is_dir {
            icon::folder(fb, gx, gy, GRID_ICON, tint, fill);
        } else {
            icon::file(fb, gx, gy, GRID_ICON, tint, fill);
        }

        let room = pw.saturating_sub(16);
        let label = truncate_to_width(fb, entry.label.trim_end_matches('/'), LABEL_PX, room);
        let lx = px + pw.saturating_sub(width_of(fb, label, LABEL_PX)) / 2;
        let ink = if lit { INK } else { INK2 };
        let _ = fb.text_ttf(lx as i32, (gy + GRID_ICON + 6) as i32, label, ink, LABEL_PX);
    }
}
