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

//! The composed canvas copied into a surface a screenshot holder shared
//! (SCREENSHOT): row by row, as much of it as both hold, ARGB as it is.
//! Pure over two surfaces, so the host proofs pin it.

use crate::sw_blitter::Surface;

/// Copy `src` into `dst` from their top left corners; the width and
/// height copied.
pub fn copy_canvas(src: Surface, dst: Surface) -> (u32, u32) {
    let (w, h) = (src.width.min(dst.width), src.height.min(dst.height));
    for y in 0..h {
        let (Some(s), Some(d)) = (src.row_start(y, 0, w), dst.row_start(y, 0, w)) else {
            return (w, y);
        };
        // SAFETY: `row_start` bounds checked both rows for `w` pixels, and a
        // client's surface is never the compositor's canvas.
        unsafe { core::ptr::copy_nonoverlapping(s as *const u32, d as *mut u32, w as usize) };
    }
    (w, h)
}
