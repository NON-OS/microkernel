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

//! An RGBA8 picture drawn scaled into a rectangle of the surface: each
//! destination pixel the area mean of the source pixels under it
//! (`blit_mean`), laid over what is there.

use nonos_toolkit::paint::resample::source_span;

use super::blit_mean::area_mean;
use super::blit_over::over;

/// Alpha-blit an RGBA8 source (bytes R,G,B,A, row-major), area-averaged into the
/// destination rect and composited over the surface. Lets the dock render a real
/// anti-aliased icon asset instead of a 1-bit bitmap. Mirrors the app-skeleton
/// blit but writes the raw surface directly: the surface's base, stride in bytes,
/// width and height; the destination rect; the source and its width and height.
pub fn blit_rgba8_scaled(
    (base_va, stride_bytes, surface_width, surface_height): (u64, u32, u32, u32),
    (dx, dy, dw, dh): (u32, u32, u32, u32),
    (rgba, sw, sh): (&[u8], u32, u32),
) {
    if dw == 0 || dh == 0 || sw == 0 || sh == 0 {
        return;
    }
    if rgba.len() < (sw as usize) * (sh as usize) * 4 {
        return;
    }
    let stride = stride_bytes as usize;
    for row in 0..dh {
        let py = dy + row;
        if py >= surface_height {
            break;
        }
        let ys = source_span(row, sh, dh);
        for col in 0..dw {
            let px = dx + col;
            if px >= surface_width {
                break;
            }
            let Some((r, g, b, a)) = area_mean(rgba, sw, sh, source_span(col, sw, dw), ys) else {
                continue;
            };
            let cell = (base_va as usize + py as usize * stride + px as usize * 4) as *mut u32;
            // SAFETY: cell is inside the surface (clipped above).
            unsafe { over(cell, r, g, b, a) };
        }
    }
}
