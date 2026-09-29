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

/* ab_glyph's PxScale is the pixel height of ascent minus descent, not the em
 * square. A CSS font-size is the em, so drawing a page font at PxScale(px)
 * paints it at units_per_em / height_unscaled of the asked size: 73% for
 * Noto Sans (1000 upm, 1362 units from ascent to descent). */

/// The PxScale that gives `f` an em of `px` pixels. A face whose head table
/// gave no units per em has no em square to convert from; its scale is then
/// the ascent-to-descent height, which is how ab_glyph measures every face.
pub fn em_scale<F: Font>(f: &F, px: f32) -> f32 {
    match f.units_per_em() {
        Some(upm) if upm > 0.0 => px * f.height_unscaled() / upm,
        _ => px,
    }
}

/// Advance of the digit zero as a fraction of the em, the CSS `ch` unit.
/// None when the face has no units per em or no glyph for '0'.
pub fn zero_advance_em<F: Font>(f: &F) -> Option<f32> {
    let upm = f.units_per_em().filter(|u| *u > 0.0)?;
    let id = f.glyph_id('0');
    if id.0 == 0 {
        return None;
    }
    Some(f.h_advance_unscaled(id) / upm)
}

/// Height of the lowercase x above the baseline as a fraction of the em, the
/// CSS `ex` unit. None when the face has no units per em or no outline for x.
pub fn x_height_em<F: Font>(f: &F) -> Option<f32> {
    let upm = f.units_per_em().filter(|u| *u > 0.0)?;
    let outline = f.outline(f.glyph_id('x'))?;
    Some(outline.bounds.max.y.max(outline.bounds.min.y) / upm)
}
