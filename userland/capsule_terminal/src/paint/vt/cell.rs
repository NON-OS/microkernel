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

//! A cell's glyph and the marks drawn over it.

use nonos_app_skeleton::PaintBuffer;
use nonos_vt::cell::attr;
use nonos_vt::{Cell, Line};

use super::area::OPAQUE;
use crate::paint::metrics::Metrics;

pub fn glyph(fb: &mut PaintBuffer, x: u32, y: u32, ch: char, argb: u32, px: f32) {
    /*
     * Blanks and controls have no glyph. Anything else goes to the face,
     * which draws .notdef for what it lacks, so a missing character shows.
     */
    if ch == ' ' || ch == '\u{a0}' || ch.is_control() {
        return;
    }
    let mut buf = [0u8; 4];
    let _ = fb.text_ttf_mono(x as i32, y as i32, ch.encode_utf8(&mut buf), argb, px);
}

/// Draw the text of `cell` at `(x, y)` in `fg`. Bold is drawn twice a pixel
/// apart: the face has no bold weight. It has no italic either, so italic
/// text is drawn upright.
pub fn text(fb: &mut PaintBuffer, line: &Line, cell: &Cell, x: u32, y: u32, fg: u32, m: Metrics) {
    let fg = OPAQUE | fg;
    // Line characters run to the cell's edges so tables join between rows.
    if super::box_draw::stroke(fb, cell.ch, x, y, m.adv, m.lh, fg) {
        return;
    }
    glyph(fb, x, y, cell.ch, fg, m.px);
    if cell.attr & attr::BOLD != 0 {
        glyph(fb, x + 1, y, cell.ch, fg, m.px);
    }
    if let Some(marks) = line.marks_of(cell) {
        for c in marks.chars() {
            glyph(fb, x, y, c, fg, m.px);
        }
    }
}
