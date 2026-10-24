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

//! The first NOTES presents, each said once: how many rectangles and the
//! box around them, the primary's serial, how many of a grid of pixels
//! sampled across the box on the screen are lit, and whether the driver
//! took the flush. A frame of an unset buffer shows as lit 0.

use alloc::format;
use alloc::string::String;

use crate::state::damage::Rect;
use crate::sw_blitter::Surface;

pub const NOTES: u8 = 3;
/// Samples a side: the grid is at most GRID by GRID.
pub const GRID: u32 = 8;

/// The box around `rects`; empty for none.
pub fn bbox(rects: &[Rect]) -> Rect {
    let zero = Rect { x: 0, y: 0, width: 0, height: 0 };
    rects
        .iter()
        .filter(|r| !r.is_empty())
        .fold(zero, |b, r| if b.is_empty() { *r } else { b.union(*r) })
}

/// Pixels sampled across `r` on `s` that are not black, and how many were
/// sampled; a sample outside `s` is not read.
pub fn lit(s: Surface, r: Rect) -> (u32, u32) {
    let (mut on, mut n) = (0, 0);
    for j in 0..GRID.min(r.height) {
        for i in 0..GRID.min(r.width) {
            let x = r.x + i * r.width / GRID.min(r.width);
            let y = r.y + j * r.height / GRID.min(r.height);
            let Some(at) = s.row_start(y, x, 1) else { continue };
            // SAFETY: row_start bounds the pixel inside the surface's
            // mapping, which the compositor holds while it presents.
            let px = unsafe { core::ptr::read_volatile(at as *const u32) };
            (on, n) = (on + (px & 0x00FF_FFFF != 0) as u32, n + 1);
        }
    }
    (on, n)
}

/// The line for present `k` (from 1).
pub fn note(k: u8, rects: &[Rect], serial: u32, lit: (u32, u32), took: bool) -> String {
    let b = bbox(rects);
    format!(
        "present {}: {} rects, box {}x{} at {},{}, serial {}, lit {} of {} sampled, {}",
        k,
        rects.len(),
        b.width,
        b.height,
        b.x,
        b.y,
        serial,
        lit.0,
        lit.1,
        if took { "taken" } else { "refused" }
    )
}
