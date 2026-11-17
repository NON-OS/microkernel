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

//! Paint one table row: the grid rules, then each cell's text clipped to the
//! column it was measured into. Cells are drawn through the unclamped `_with`
//! font entry point, the same one the measurer used, so paint matches layout.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf::{builtin_face, draw_text_sheared};

use super::sheet_scale::on_screen;
use super::state::State;
use super::table_rules::paint_rules;
use super::theme;
use crate::doc::linebox::LineBox;
use crate::doc::style::Family;
use crate::doc::table::fit::fit;
use crate::doc::table::geom::{col_x, CELL_PAD_X, ROW_PAD_Y};
use crate::doc::table::run::run_of;
use crate::doc::table::syntax::cell_span;
use crate::doc::table::widths::col_widths;
use crate::doc::ttf_measure::TtfMeasurer;

pub(super) fn paint_row(
    state: &State,
    fb: &mut PaintBuffer,
    line: &LineBox,
    origin: (i32, i32),
    tail: bool,
) {
    let Some(b) = state.doc.blocks.get(line.block) else { return };
    let Some(run) = run_of(&state.doc, line.block) else { return };
    let cw = state.page_metrics.content_width();
    let w = col_widths(&state.doc, run, cw, &TtfMeasurer);
    if w.is_empty() {
        return;
    }
    let (ox, y) = (origin.0, origin.1 + on_screen(line.y) as i32);
    let h = on_screen(line.height.max(1.0)) as u32;
    let sw: Vec<f32> = w.iter().map(|&c| on_screen(c)).collect();
    paint_rules(fb, ox, y, &sw, h, tail || line.block + 1 == run.1);
    let (t, st) = (b.as_str(), b.style_at(0));
    let Some(f) = builtin_face(st.family == Family::Mono, st.bold) else { return };
    let (fw, fh, stride) = (fb.width, fb.height, fb.stride_words as usize);
    let ink = theme::active().ink(st.color);
    for (c, cwidth) in w.iter().enumerate() {
        let Some((s, e)) = cell_span(t, c) else { break };
        let text = fit(t.get(s..e).unwrap_or(""), cwidth - 2.0 * CELL_PAD_X, &st, &TtfMeasurer);
        let tx = ox + (col_x(&sw, c) + on_screen(CELL_PAD_X)) as i32;
        let ty = y + on_screen(ROW_PAD_Y) as i32;
        let px = on_screen(st.size_px);
        draw_text_sheared(f, fb.pixels, stride, fw, fh, tx, ty, text, ink, px, 0.0, 0.0);
    }
}
