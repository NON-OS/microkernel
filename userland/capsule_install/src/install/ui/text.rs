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

//! Text placement over the toolkit's TrueType rasteriser. Every line in
//! this window goes through here, so the metrics are decided once.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::ttf::line_height;

pub fn line(fb: &mut PaintBuffer, x: u32, top: u32, s: &str, argb: u32, px: f32) {
    fb.text_ttf(x as i32, top as i32, s, argb, px);
}

pub fn mono(fb: &mut PaintBuffer, x: u32, top: u32, s: &str, argb: u32, px: f32) {
    fb.text_ttf_mono(x as i32, top as i32, s, argb, px);
}

pub fn width(fb: &PaintBuffer, s: &str, px: f32) -> u32 {
    fb.measure_ttf(s, px).max(0) as u32
}

pub fn right(fb: &mut PaintBuffer, right_x: u32, top: u32, s: &str, argb: u32, px: f32) {
    let w = width(fb, s, px);
    line(fb, right_x.saturating_sub(w), top, s, argb, px);
}

/// The top of a line box centred in a row of height `h`.
pub fn top_of(y: u32, h: u32, px: f32) -> u32 {
    y + h.saturating_sub(line_height(px).max(1) as u32) / 2
}

/// Longest prefix that measures within `max_w`, cut on a char boundary.
/// Never by glyph count: the face is proportional.
pub fn fit<'a>(fb: &PaintBuffer, s: &'a str, px: f32, max_w: u32) -> &'a str {
    let mut end = s.len();
    while end > 0 {
        if s.is_char_boundary(end) && width(fb, &s[..end], px) <= max_w {
            return &s[..end];
        }
        end -= 1;
    }
    ""
}
