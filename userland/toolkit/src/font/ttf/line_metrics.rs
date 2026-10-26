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

//! A line's height and ascent, in the text face at the readable size or in
//! a face the caller gives.

use ab_glyph::{Font, PxScale, ScaleFont};

use super::face::face;
use super::readable::readable_px;

// Baseline-to-baseline distance at `px`.
pub fn line_height(px: f32) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(false) else { return px as i32 };
    let sf = f.as_scaled(PxScale::from(px));
    (sf.ascent() - sf.descent() + sf.line_gap()) as i32
}

// Same measurement with a caller-provided face, matching measure_with. Unlike
// line_height, px is not clamped to the readable floor.
pub fn line_height_with<F: Font>(f: &F, px: f32) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    (sf.ascent() - sf.descent() + sf.line_gap()) as i32
}

// Top-of-line to baseline at `px`.
pub fn ascent(px: f32) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(false) else { return px as i32 };
    f.as_scaled(PxScale::from(px)).ascent() as i32
}

// Same measurement with a caller-provided face, matching measure_with. Unlike
// ascent, px is not clamped to the readable floor.
pub fn ascent_with<F: Font>(f: &F, px: f32) -> i32 {
    f.as_scaled(PxScale::from(px)).ascent() as i32
}
