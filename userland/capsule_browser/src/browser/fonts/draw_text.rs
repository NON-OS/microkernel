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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::em::em_scale;
use nonos_toolkit::font::ttf::{self, FontRef, OBLIQUE};

use super::run_face::with_pieces;
use super::text::TextRun;

/* Draw the run with the resolved faces. Returns true when a true bold cut
 * was used, so the caller knows the fake thickening is not needed. A piece
 * in a fallback face shares the run's baseline. */
pub fn draw_text(fb: &mut PaintBuffer, run: TextRun, text: &str, argb: u32) -> bool {
    let stride = fb.stride_words as usize;
    let (w, h) = (fb.width, fb.height);
    let TextRun { key, mono, bold, italic, x, top_y, px, spacing, clip } = run;
    let slant = if italic { OBLIQUE } else { 0.0 };
    let mut pen = x;
    let paint = |f: &FontRef, piece: &str, primary: &FontRef| {
        let scale = em_scale(f, px);
        let lift = ttf::ascent_with(primary, em_scale(primary, px)) - ttf::ascent_with(f, scale);
        let y = top_y + lift;
        let style = (scale, spacing, slant);
        pen = ttf::draw_text_clipped(
            f,
            fb.pixels,
            stride,
            (w, h),
            clip,
            (pen, y),
            piece,
            argb,
            style,
        );
    };
    with_pieces(key, mono, bold, text, paint).unwrap_or(false)
}
