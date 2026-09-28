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

/*
 * One built-in face standing in for the other. Noto Sans has no arrows,
 * no triangles and no not-equal sign; its mono cut has them. A character
 * the chosen face lacks is drawn from the other rather than as a box,
 * which reads as a bug in whatever app printed it.
 */

use ab_glyph::{Font, FontRef, GlyphId, PxScale, ScaleFont};

use super::face::face;
use super::readable::readable_px;

/// The face that draws `ch`, as (is it the mono face, the face): the one
/// asked for when it has the glyph, the other when only that one does.
pub(super) fn face_for(
    f: &'static FontRef<'static>,
    mono: bool,
    ch: char,
) -> (bool, &'static FontRef<'static>) {
    if f.glyph_id(ch).0 != 0 {
        return (mono, f);
    }
    match face(!mono) {
        Some(other) if other.glyph_id(ch).0 != 0 => (!mono, other),
        _ => (mono, f),
    }
}

/*
 * The advance of `text` in the built-in faces, each glyph measured in the
 * face that will draw it. Kerning only pairs glyphs of one face.
 */
pub fn measure(text: &str, px: f32, mono: bool) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(mono) else { return 0 };
    let mut pen = 0.0f32;
    let mut prev: Option<(bool, GlyphId)> = None;
    for ch in text.chars() {
        let (m, gf) = face_for(f, mono, ch);
        let sf = gf.as_scaled(PxScale::from(px));
        let g = sf.glyph_id(ch);
        if let Some((_, p)) = prev.filter(|(pm, _)| *pm == m) {
            pen += sf.kern(p, g);
        }
        pen += sf.h_advance(g);
        prev = Some((m, g));
    }
    pen as i32
}
