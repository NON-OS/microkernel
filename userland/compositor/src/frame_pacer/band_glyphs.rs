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
//! A band mark drawn: its glyph (band_glyph_art.rs) in the mark's colour,
//! on or dim, inside the damaged rectangle only.

use super::band_meet::meet;
use crate::state::damage::Rect;
use crate::sw_blitter::{fill_rect, Surface};

pub use super::band_glyph_art::{Glyph, CAMERA, LOCK, MICROPHONE, SCREEN, SIDE};

/// Whether the glyph's pixel (x, y) is set.
pub fn set(g: &Glyph, x: u32, y: u32) -> bool {
    x < SIDE && y < SIDE && g[y as usize] >> (SIDE - 1 - x) & 1 == 1
}

/// Draw `g` at (x, y) in `argb`, inside `clip`.
pub fn draw_glyph(dst: Surface, g: &Glyph, (x, y): (u32, u32), argb: u32, clip: Rect) {
    for gy in 0..SIDE {
        for gx in (0..SIDE).filter(|gx| set(g, *gx, gy)) {
            let px = Rect { x: x + gx, y: y + gy, width: 1, height: 1 };
            if let Some(r) = meet(px, clip) {
                fill_rect(dst, r, argb);
            }
        }
    }
}
