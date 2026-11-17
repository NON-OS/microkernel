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

//! The Launchpad's label as the shared `Label` reads as the shell's own text
//! did. The scale test asks at each scale it sets, so the scale holds still.

use nonos_toolkit::font::ttf;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Align, Label, Rect};

use crate::shell_layer_tests::{changed, painted, NAME, STRIDE};
use crate::ui_font::{scaled, top_y_centered, UI_PX};

/// The label drawn as the shared `Label`, and the name drawn as the shell drew
/// it before, match pixel for pixel at the scale in force.
pub(crate) fn label_reads_as_shell_text(tag: &str) {
    let band = Rect { x: 8, y: 8, w: 140, h: 100 };
    let shared = painted(|fb| Label::new(NAME).align(Align::Centre).draw(fb, band));
    let (px, ink) = (scaled(UI_PX), roles().text.argb());
    let x = band.x + band.w.saturating_sub(ttf::measure(NAME, px, false).max(0) as u32) / 2;
    let top = top_y_centered(band.y, band.h, UI_PX) as i32;
    let shell = painted(|fb| {
        let (w, h) = (fb.width, fb.height);
        ttf::draw_text(fb.pixels, STRIDE as usize, w, h, x as i32, top, NAME, ink, px, false);
    });
    assert!(shared == shell && !changed(&shell).is_empty(), "{tag}: the label differs");
}
