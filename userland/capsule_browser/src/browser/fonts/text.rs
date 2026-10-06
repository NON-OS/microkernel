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

use nonos_toolkit::font::em::em_scale;
use nonos_toolkit::font::ttf;

use super::run_face::{with_pieces, with_run_face};

/* Bold cuts store under the family key with this bit set, so one registry
 * holds both weights without a second key namespace. */
pub const BOLD_KEY: u32 = 1 << 31;

/* A TextRun clip that cuts nothing. */
pub const NO_CLIP: [i32; 4] = [i32::MIN, i32::MIN, i32::MAX, i32::MAX];

/* One text run's face and geometry, grouped so calls stay readable. */
pub struct TextRun {
    pub key: u32,
    pub mono: bool,
    pub bold: bool,
    /* Drawn slanted, font-style italic or oblique; advances stay upright. */
    pub italic: bool,
    pub x: i32,
    pub top_y: i32,
    pub px: f32,
    pub spacing: f32,
    /* Screen [x0, y0, x1, y1] the glyphs may cover; the rest is cut. */
    pub clip: [i32; 4],
}

/* Advance width at a font-size of `px`, the CSS em. Page text keeps its
 * declared size (the chrome's readability floor is for UI text only), and
 * draw_text paints at this same size, piece by piece in the same faces. */
pub fn measure_text(key: u32, mono: bool, bold: bool, text: &str, px: f32, spacing: f32) -> i32 {
    let mut w = 0;
    with_pieces(key, mono, bold, text, |f, piece, _| {
        w += ttf::measure_tracked(f, piece, em_scale(f, px), spacing);
    });
    w
}

/* Ascent-to-descent height at a font-size of `px`: the CSS content area a
 * line's half-leading is split around. */
pub fn content_height(key: u32, mono: bool, bold: bool, px: f32) -> f32 {
    with_run_face(key, mono, bold, |f| em_scale(f, px)).map_or(px, |(h, _)| h)
}
