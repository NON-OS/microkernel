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

use ab_glyph::{point, Font, Glyph, GlyphId, PxScale, ScaleFont};

use super::raster::box_area;
use super::target::Target;

/* The slanted path of draw_text_sheared: each glyph is outlined at its exact
pen and its coverage rows are pushed right by `slant` per pixel above the
baseline. Nothing is cached, as the shear depends on the exact position. A
glyph box over the surface's limit draws nothing. */
pub(super) fn draw_sheared<F: Font>(
    f: &F,
    t: &mut Target,
    (x, top_y): (i32, i32),
    text: &str,
    argb: u32,
    px: f32,
    (spacing, slant): (f32, f32),
) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    let baseline = top_y as f32 + sf.ascent();
    let mut pen = x as f32;
    let mut prev: Option<GlyphId> = None;
    for ch in text.chars() {
        let mut g: Glyph = sf.scaled_glyph(ch);
        if let Some(p) = prev {
            pen += sf.kern(p, g.id);
        }
        g.position = point(pen, baseline);
        let adv = sf.h_advance(g.id);
        prev = Some(g.id);
        if let Some(og) = sf.outline_glyph(g) {
            let bb = og.px_bounds();
            if box_area(bb.width(), bb.height()) <= t.glyph_limit() {
                og.draw(|dx, dy, c| {
                    let py = bb.min.y as i32 + dy as i32;
                    let shear = (baseline - py as f32) * slant;
                    let x = bb.min.x as i32 + dx as i32 + shear as i32;
                    t.blend(x, py, argb, (c * 255.0 + 0.5) as u8);
                });
            }
        }
        pen += adv + spacing;
    }
    pen as i32
}
