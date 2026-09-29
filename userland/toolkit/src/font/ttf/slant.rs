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

use ab_glyph::Font;

use super::sheared::draw_sheared;
use super::target::Target;
use super::upright::draw_upright;

/* Faux-oblique slant for faces that ship no italic cut: each pixel row is
 * pushed right in proportion to its height above the baseline, about 12
 * degrees. Only coverage moves; advances and kerning are untouched, so a
 * sheared run measures exactly as its upright one does. */
pub const OBLIQUE: f32 = 0.22;

/* Tracked rendering with the glyph coverage sheared by `slant`; a slant of
 * 0.0 is the upright path, served from the glyph cache. The returned pen x
 * still matches `measure_with`, so underline rules, hit testing and the caret
 * stay on the drawn glyphs. The face must expose its font data
 * (`Font::font_data`), as FontRef, FontVec and FontArc do: the glyph cache
 * knows a face by that data. */
pub fn draw_text_sheared<F: Font>(
    f: &F,
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: i32,
    top_y: i32,
    text: &str,
    argb: u32,
    px: f32,
    spacing: f32,
    slant: f32,
) -> i32 {
    let mut t = Target { buf, stride, w, h, clip: [0, 0, w as i64, h as i64] };
    if slant == 0.0 {
        return draw_upright(f, &mut t, (x, top_y), text, argb, px, spacing);
    }
    draw_sheared(f, &mut t, (x, top_y), text, argb, px, (spacing, slant))
}
