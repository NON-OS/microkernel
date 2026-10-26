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
 * A character the chosen face lacks is drawn from another built-in face
 * rather than as a box, which reads as a bug in whatever app printed it:
 * first Noto Sans, which carries the wider Latin, Greek and Cyrillic Geist
 * lacks, then the other of Geist and JetBrains Mono, which between them
 * carry the arrows, triangles and signs.
 */

use ab_glyph::{Font, FontRef, GlyphId, PxScale, ScaleFont};

use super::face::{face, wide};
use super::readable::readable_px_for;

/// The face that draws `ch`: the one asked for when it has the glyph, else
/// the first of Noto Sans and the other built-in face that has it, else the
/// one asked for, which draws its own box.
pub(super) fn face_for(
    f: &'static FontRef<'static>,
    mono: bool,
    ch: char,
) -> &'static FontRef<'static> {
    let has = |g: &FontRef| g.glyph_id(ch).0 != 0;
    if has(f) {
        return f;
    }
    [wide(), face(!mono)].into_iter().flatten().find(|g| has(g)).unwrap_or(f)
}

/*
 * The advance of `text` in the built-in faces, each glyph measured in the
 * face that will draw it. Kerning only pairs glyphs of one face.
 */
pub fn measure(text: &str, px: f32, mono: bool) -> i32 {
    let px = readable_px_for(px, mono);
    let Some(f) = face(mono) else { return 0 };
    let mut pen = 0.0f32;
    let mut prev: Option<(*const FontRef<'static>, GlyphId)> = None;
    for ch in text.chars() {
        let gf = face_for(f, mono, ch);
        let sf = gf.as_scaled(PxScale::from(px));
        let g = sf.glyph_id(ch);
        if let Some((_, p)) = prev.filter(|(pf, _)| core::ptr::eq(*pf, gf)) {
            pen += super::gpos::kern::kern_px(&sf, p, g);
        }
        pen += sf.h_advance(g);
        prev = Some((gf, g));
    }
    pen as i32
}
