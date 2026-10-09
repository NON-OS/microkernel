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

//! Text in the brand's faces. Labels are JetBrains Mono in capitals with
//! wide tracking, the way the site sets them.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf::{draw_text_tracked, line_height_with, measure_tracked};

use super::faces::{font, Face};

/// Draw `s` with its line box's top left at (x, top); returns the pen x.
pub fn text(
    fb: &mut PaintBuffer,
    x: u32,
    top: u32,
    s: &str,
    face: Face,
    argb: u32,
    px: f32,
) -> u32 {
    spaced(fb, x, top, s, face, argb, px, 0.0)
}

// Each argument is one input of the tracked draw call, passed straight through.
#[allow(clippy::too_many_arguments)]
pub fn spaced(
    fb: &mut PaintBuffer,
    x: u32,
    top: u32,
    s: &str,
    face: Face,
    argb: u32,
    px: f32,
    sp: f32,
) -> u32 {
    let Some(f) = font(face) else { return x };
    let (w, h, stride) = (fb.width, fb.height, fb.stride_words as usize);
    draw_text_tracked(&f, fb.pixels, stride, w, h, x as i32, top as i32, s, argb, px, sp).max(0)
        as u32
}

pub fn measure(s: &str, face: Face, px: f32, sp: f32) -> u32 {
    font(face).map_or(0, |f| measure_tracked(&f, s, px, sp).max(0) as u32)
}

pub fn line_h(face: Face, px: f32) -> u32 {
    font(face).map_or(px as u32, |f| line_height_with(&f, px).max(1) as u32)
}
