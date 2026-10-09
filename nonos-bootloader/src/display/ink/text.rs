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

//! Anti-aliased text over whatever is already on screen: each glyph pixel
//! blends the framebuffer's colour toward the text colour by its coverage.

use super::atlas::Face;
use super::glyph::{coverage, glyph, Glyph};
use super::style::{face_of, Style};
use crate::display::fx::mix;
use crate::display::gop::{get_pixel, put_pixel};

/// Draw `s` with its line box's top left at (x, y). Returns its width.
pub fn draw(x: u32, y: u32, s: &[u8], style: Style, color: u32) -> u32 {
    let Some(f) = face_of(style) else { return 0 };
    let mut pen64 = (x as u64) << 6;
    for &code in s {
        let Some(g) = glyph(&f, code) else { continue };
        stamp(((pen64 + 32) >> 6) as i64 + g.left as i64, y as i64 + g.top as i64, &g, color, 256);
        pen64 += g.adv64 as u64;
    }
    (((pen64 + 32) >> 6) as u32).saturating_sub(x)
}

/// Blend glyph `g` at (gx, gy) toward `color`, its coverage scaled by
/// `gain` / 256, over what is already on screen.
pub fn stamp(gx: i64, gy: i64, g: &Glyph, color: u32, gain: u32) {
    for row in 0..g.h {
        for col in 0..g.w {
            let a = coverage(g, col, row) * 17 * gain / 256;
            let (px, py) = (gx + col as i64, gy + row as i64);
            if a == 0 || px < 0 || py < 0 {
                continue;
            }
            let (px, py) = (px as u32, py as u32);
            let c = if a >= 255 { color } else { mix(get_pixel(px, py), color, a) };
            put_pixel(px, py, c);
        }
    }
}

/// The width `s` takes in `style`.
pub fn width(s: &[u8], style: Style) -> u32 {
    let Some(f) = face_of(style) else { return 0 };
    let adv: u64 = s.iter().filter_map(|&c| glyph(&f, c)).map(|g| g.adv64 as u64).sum();
    ((adv + 32) >> 6) as u32
}

/// Draw `s` centred in the column at `x`, `w` wide.
pub fn draw_centered(x: u32, w: u32, y: u32, s: &[u8], style: Style, color: u32) {
    draw(x + w.saturating_sub(width(s, style)) / 2, y, s, style, color);
}

/// Line height and ascent of `style`, from the atlas.
pub fn metrics(style: Style) -> Face {
    face_of(style).unwrap_or(Face::EMPTY)
}
