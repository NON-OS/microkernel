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

use super::draw::draw_text_tracked;
use super::face::face;
use super::readable::readable_px;

/* Same rendering with a caller-provided face, so text can draw in a font
 * loaded at runtime, such as a page's web font. */
pub fn draw_text_with<F: Font>(
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
) -> i32 {
    draw_text_tracked(f, buf, stride, w, h, x, top_y, text, argb, px, 0.0)
}

/* Tracked rendering with the built-in faces, the fallback while a page font
 * is still loading. */
pub fn draw_text_spaced(
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
    spacing: f32,
) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(mono) else { return x };
    draw_text_tracked(f, buf, stride, w, h, x, top_y, text, argb, px, spacing)
}
