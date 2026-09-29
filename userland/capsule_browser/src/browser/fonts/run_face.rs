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

use nonos_toolkit::font::fallback::{fallback_face, has_glyph};
use nonos_toolkit::font::ttf::{self, FontRef};

use super::registry::with_face;
use super::text::BOLD_KEY;

/* Run `f` on the run's face: the family's bold cut, its regular one, then the
 * built-in face. The flag is true when the face carries the weight itself. */
pub(super) fn with_run_face<R>(
    key: u32,
    mono: bool,
    bold: bool,
    mut f: impl FnMut(&FontRef) -> R,
) -> Option<(R, bool)> {
    if bold {
        if let Some(r) = with_face(key | BOLD_KEY, &mut f) {
            return Some((r, true));
        }
    }
    if let Some(r) = with_face(key, &mut f) {
        return Some((r, false));
    }
    ttf::builtin_face(mono, bold).map(|face| (f(face), bold && !mono))
}

/// Call `f(face, piece, primary)` on each piece of `text` that draws in one
/// face, in order: the run's own face `primary`, or for characters it has
/// no glyph for, the fallback face that has one. Measure and draw both walk
/// these pieces, so their widths agree. Returns whether the run's face is a
/// true bold cut, None when there is no face at all.
pub(super) fn with_pieces(
    key: u32,
    mono: bool,
    bold: bool,
    text: &str,
    mut f: impl FnMut(&FontRef, &str, &FontRef),
) -> Option<bool> {
    let walk = |primary: &FontRef| {
        let face_of = |ch: char| fallback_face(ch).filter(|_| !has_glyph(primary, ch));
        let mut start = 0;
        let mut cur: Option<&FontRef> = None;
        for (i, ch) in text.char_indices() {
            let face = face_of(ch);
            if face.map(|p| p as *const FontRef) != cur.map(|p| p as *const FontRef) {
                if i > start {
                    f(cur.unwrap_or(primary), &text[start..i], primary);
                }
                (start, cur) = (i, face);
            }
        }
        if start < text.len() {
            f(cur.unwrap_or(primary), &text[start..], primary);
        }
    };
    with_run_face(key, mono, bold, walk).map(|((), real_bold)| real_bold)
}
