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

use super::chrome::draw_cached;
use super::face::face;
use super::readable::readable_px;
use super::target::Target;

/* Render `text` with its top-left at (x, top_y) and return the pen x after
 * the last glyph. `px` is the em size in pixels. Kerning is applied between
 * adjacent glyphs; each glyph is rasterized once and cached, then blended. */
pub fn draw_text(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: i32,
    top_y: i32,
    text: &str,
    argb: u32,
    px: f32,
    mono: bool,
) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(mono) else { return x };
    let mut t = Target { buf, stride, w, h, clip: [0, 0, w as i64, h as i64] };
    draw_cached(f, mono, &mut t, (x, top_y), text, argb, px)
}

/* Same rendering with extra advance between glyphs, for letter-spacing. */
pub fn draw_text_tracked<F: Font>(
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
) -> i32 {
    super::slant::draw_text_sheared(f, buf, stride, w, h, x, top_y, text, argb, px, spacing, 0.0)
}
