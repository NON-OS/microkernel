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

use ab_glyph::{Font, FontRef};
use spin::Once;

/* Faces for scripts the built-in Noto Sans has no glyphs for, each subset
 * to its script's Unicode blocks. All OFL; see the NotoSans*-OFL.txt notices
 * beside them in assets/fonts. CJK and Hangul are not carried: their faces
 * run to megabytes. */
const ARABIC: &[u8] = include_bytes!("../../assets/fonts/NotoSansArabic-Subset.ttf");
const HEBREW: &[u8] = include_bytes!("../../assets/fonts/NotoSansHebrew-Subset.ttf");
const DEVANAGARI: &[u8] = include_bytes!("../../assets/fonts/NotoSansDevanagari-Subset.ttf");

static FACES: Once<[Option<FontRef<'static>>; 3]> = Once::new();

/* Which face covers a code point: the index into FACES by Unicode block. */
fn script(ch: char) -> Option<usize> {
    Some(match ch as u32 {
        0x0600..=0x06FF | 0x0750..=0x077F | 0x08A0..=0x08FF => 0,
        0xFB50..=0xFDFF | 0xFE70..=0xFEFF => 0,
        0x0590..=0x05FF | 0xFB1D..=0xFB4F => 1,
        0x0900..=0x097F | 0x1CD0..=0x1CFF | 0xA8E0..=0xA8FF => 2,
        _ => return None,
    })
}

/// The fallback face that draws `ch`, when `ch` belongs to a script a
/// fallback carries and that face has a glyph for it. Latin and every other
/// block outside those scripts answer None without touching a face.
pub fn fallback_face(ch: char) -> Option<&'static FontRef<'static>> {
    let i = script(ch)?;
    let faces = FACES.call_once(|| {
        [ARABIC, HEBREW, DEVANAGARI].map(|bytes| FontRef::try_from_slice(bytes).ok())
    });
    faces[i].as_ref().filter(|f| f.glyph_id(ch).0 != 0)
}

/// Whether `f` has a glyph of its own for `ch`, not the .notdef box.
pub fn has_glyph<F: Font>(f: &F, ch: char) -> bool {
    f.glyph_id(ch).0 != 0
}
