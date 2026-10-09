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

//! Lines drawn across a cell: the underline styles, strikethrough and
//! overline.

use nonos_app_skeleton::PaintBuffer;
use nonos_vt::cell::attr;
use nonos_vt::{Cell, Underline};

use super::area::OPAQUE;
use crate::paint::metrics::Metrics;

fn curly(fb: &mut PaintBuffer, x: u32, base: u32, w: u32, fg: u32) {
    let pts: alloc::vec::Vec<(i32, i32)> = (0..=w / 2)
        .map(|i| {
            let py = if i % 2 == 0 { base.saturating_sub(1) } else { base + 1 };
            ((x + 2 * i) as i32, py as i32)
        })
        .collect();
    fb.polyline(&pts, fg);
}

pub fn decorations(fb: &mut PaintBuffer, cell: &Cell, x: u32, y: u32, w: u32, fg: u32, m: Metrics) {
    let fg = OPAQUE | fg;
    let base = y + m.lh.saturating_sub(3);
    match cell.underline() {
        Underline::None => {}
        Underline::Single => fb.fill_rect(x, base, w, 1, fg),
        Underline::Double => {
            fb.fill_rect(x, base.saturating_sub(2), w, 1, fg);
            fb.fill_rect(x, base, w, 1, fg);
        }
        Underline::Curly => curly(fb, x, base, w, fg),
        Underline::Dotted => (0..w).step_by(2).for_each(|i| fb.fill_rect(x + i, base, 1, 1, fg)),
        Underline::Dashed => {
            (0..w).step_by(5).for_each(|i| fb.fill_rect(x + i, base, 3.min(w - i), 1, fg))
        }
    }
    if cell.attr & attr::STRIKE != 0 {
        fb.fill_rect(x, y + m.lh / 2, w, 1, fg);
    }
    if cell.attr & attr::OVERLINE != 0 {
        fb.fill_rect(x, y + 1, w, 1, fg);
    }
}
