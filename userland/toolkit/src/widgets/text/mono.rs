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

//! Measuring, placing and cutting one line in the monospace face, for columns
//! of digits and addresses that must line up cell for cell. The mono face has
//! no `Surface` primitive, so the painters take a `PaintBuffer`.

use alloc::string::String;

use super::fit::truncate_by;
use crate::paint::{measure_ttf_mono, PaintBuffer};
use crate::theme::Opaque;

/// The advance width of `s` at `px` in the mono face, clamped at zero. It
/// measures with the same function `mono_left` draws with.
pub fn mono_width(s: &str, px: f32) -> u32 {
    measure_ttf_mono(s, px).max(0) as u32
}

/// Draws `s` in the mono face with its left edge at `x` and `top` the top of
/// the line box. Returns the pen x after the last glyph.
pub fn mono_left(fb: &mut PaintBuffer<'_>, x: u32, top: i32, s: &str, ink: Opaque, px: f32) -> i32 {
    fb.text_ttf_mono(i32::try_from(x).unwrap_or(i32::MAX), top, s, ink.argb(), px)
}

/// Draws `s` in the mono face so its measured width ends at `right_x`, with
/// `top` the top of the line box, so a column of numbers ends on its last
/// digit. Returns the x it drew at, clamped at zero.
pub fn mono_right(
    fb: &mut PaintBuffer<'_>,
    right_x: u32,
    top: i32,
    s: &str,
    ink: Opaque,
    px: f32,
) -> u32 {
    let x = right_x.saturating_sub(mono_width(s, px));
    mono_left(fb, x, top, s, ink, px);
    x
}

/// `truncate_by` measured in the mono face at `px`, so a run is cut against
/// the advance it is drawn with.
pub fn truncate_mono(s: &str, max_w: u32, px: f32) -> String {
    truncate_by(s, i32::try_from(max_w).unwrap_or(i32::MAX), |t| measure_ttf_mono(t, px))
}
