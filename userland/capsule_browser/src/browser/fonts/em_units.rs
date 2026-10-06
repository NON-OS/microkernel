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

use nonos_toolkit::font::em::{x_height_em, zero_advance_em};
use nonos_toolkit::font::ttf::builtin_face;
use spin::Once;

/* The font-relative units ch and ex, as fractions of the em. Lengths are
 * resolved while cascading, before the element's own web font is known to
 * have loaded, so they are read from the built-in body face: the face the
 * text is drawn in until a web font arrives. CSS Values 4 section 6.1.1
 * specifies 0.5em for both when the metric cannot be read from a face; that
 * is used only when the built-in face has no '0' or 'x' glyph. */
const SPEC_FALLBACK_EM: f32 = 0.5;

static RATIOS: Once<(f32, f32)> = Once::new();

fn ratios() -> (f32, f32) {
    *RATIOS.call_once(|| match builtin_face(false, false) {
        Some(f) => (
            zero_advance_em(f).unwrap_or(SPEC_FALLBACK_EM),
            x_height_em(f).unwrap_or(SPEC_FALLBACK_EM),
        ),
        None => (SPEC_FALLBACK_EM, SPEC_FALLBACK_EM),
    })
}

/// One `ch` in px at a font-size of `em` px: the advance of the digit zero.
pub fn ch_px(em: f32) -> f32 {
    ratios().0 * em
}

/// One `ex` in px at a font-size of `em` px: the height of the lowercase x.
pub fn ex_px(em: f32) -> f32 {
    ratios().1 * em
}
