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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::fonts::{content_height, draw_text, TextRun};
use crate::browser::layout::boxmodel::{Content, Fragment};

use super::box_page::TOP;
use super::fill_page::fill_page;

/* A text fragment on screen row `sy`. The font's content area (ascent to
 * descent at the true em size) sits centred in the line box, the half
 * leading above and below it, which puts the baseline where CSS does. */
pub(super) fn paint_text(fb: &mut PaintBuffer, f: &Fragment, sy: i32, clip: Option<[i32; 4]>) {
    let Content::Text { text, color, px, bold, mono, underline, font, spacing, italic } =
        &f.content
    else {
        return;
    };
    let color = super::fade::fade(*color, f.alpha);
    let area = content_height(*font, *mono, *bold, *px);
    /* Half the leading above; a line box shorter than the content area
     * makes it negative, and the glyphs overhang the box equally. Glyphs
     * may not reach up into the browser chrome, which is painted already. */
    let ty = sy + (f.h - (area + 0.5) as i32) / 2;
    if ty < TOP {
        return;
    }
    let run = |x: i32| TextRun {
        key: *font,
        mono: *mono,
        bold: *bold,
        italic: *italic,
        x,
        top_y: ty,
        px: *px,
        spacing: *spacing,
    };
    /* A true bold cut needs no thickening; the fake second pass only
     * remains for faces that never shipped one, the mono cut mostly. */
    let real_bold = draw_text(fb, run(f.x), text, color);
    if *bold && !real_bold {
        draw_text(fb, run(f.x + 1), text, color);
    }
    if *underline {
        fill_page(fb, f.x, sy + f.h - 2, f.w, 1, color, clip);
    }
}
