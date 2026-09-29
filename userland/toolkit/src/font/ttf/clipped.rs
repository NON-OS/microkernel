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

/// draw_text_sheared confined to `clip`, [x0, y0, x1, y1] in surface pixels:
/// coverage outside it is dropped, so a run cut by a scroll edge or by an
/// overflow box paints the part that shows. `size` is the surface (w, h),
/// `at` the pen and top, `style` (px, spacing, slant). The pen x returned is
/// the unclipped one, as draw_text_sheared returns it.
pub fn draw_text_clipped<F: Font>(
    f: &F,
    buf: &mut [u32],
    stride: usize,
    size: (u32, u32),
    clip: [i32; 4],
    at: (i32, i32),
    text: &str,
    argb: u32,
    style: (f32, f32, f32),
) -> i32 {
    let (w, h) = size;
    let [x0, y0, x1, y1] = clip.map(|v| v.max(0) as i64);
    let clip = [x0, y0, x1.min(w as i64), y1.min(h as i64)];
    let mut t = Target { buf, stride, w, h, clip };
    let (px, spacing, slant) = style;
    if slant == 0.0 {
        return draw_upright(f, &mut t, at, text, argb, px, spacing);
    }
    draw_sheared(f, &mut t, at, text, argb, px, (spacing, slant))
}
