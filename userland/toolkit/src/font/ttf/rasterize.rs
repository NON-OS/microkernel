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

use ab_glyph::{Font, Glyph, OutlinedGlyph, ScaleFont};

use super::raster::{floor, Raster};

/* How far a glyph's box is pushed out left and right, in pixels. */
const PAD_PX: f32 = 1.0 / 16.0;

/*
 * The glyph outlined for a pen with a subpixel fraction, its box pushed out
 * by PAD_PX left and right. ab_glyph floors the box's left edge from the
 * outline's scaled bound plus the pen fraction, and the rasteriser places the
 * same edge by a different sum. When an edge falls on a whole pixel
 * (JetBrains Mono's T at 12 px, at a half-pixel pen) the two disagree in the
 * last bit: the box starts at 1.0 and the edge lands at -0.0000001, a cell
 * outside its row, and every running sum after it is wrong, so the glyph
 * comes out as broken bars. With the pad no edge sits on the box's boundary;
 * the box is at most a pixel wider, all of it zero coverage, and the glyph
 * lands where it did. At a whole-pixel pen the two sums agree exactly, so
 * the glyph keeps ab_glyph's own box, as the chrome path's always do.
 */
pub(super) fn outlined<F: Font, S: ScaleFont<F>>(sf: &S, g: Glyph) -> Option<OutlinedGlyph> {
    let mut o = sf.font().outline(g.id)?;
    let pad = PAD_PX / sf.h_scale_factor();
    if g.position.x != floor(g.position.x) && pad.is_finite() {
        o.bounds.min.x -= pad;
        o.bounds.max.x += pad;
    }
    Some(OutlinedGlyph::new(g, o, sf.scale_factor()))
}

/* Coverage for an outlined glyph, one byte per pixel of its box; `bias` is
added before truncating each sample, so 0.5 rounds and 0.0 truncates. */
pub(super) fn rasterize(og: &OutlinedGlyph, bias: f32) -> Option<Raster> {
    let bb = og.px_bounds();
    let (min_x, min_y) = (bb.min.x as i32, bb.min.y as i32);
    let w = (bb.max.x as i32 - min_x).max(0) as u32;
    let h = (bb.max.y as i32 - min_y).max(0) as u32;
    if w == 0 || h == 0 {
        return None;
    }
    let mut cov = alloc::vec![0u8; w as usize * h as usize];
    og.draw(|dx, dy, c| {
        if let Some(p) = cov.get_mut(dy as usize * w as usize + dx as usize) {
            *p = (c * 255.0 + bias) as u8;
        }
    });
    Some(Raster { min_x, min_y, w, h, cov })
}
