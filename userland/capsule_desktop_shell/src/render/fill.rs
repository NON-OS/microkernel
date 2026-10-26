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

//! A rectangle filled with one colour, clipped to the surface.

use crate::render::layout::Rect;
use crate::state::Context;

/// Fill `r` on the context's overlay backing.
pub fn fill_rect(ctx: &Context, r: Rect, argb: u32) {
    let (base_va, stride_bytes) = (ctx.backing_va, ctx.stride);
    let (surface_width, surface_height) = (ctx.width, ctx.height);
    let (rect_x, rect_y, rect_w, rect_h) = (r.x, r.y, r.width, r.height);
    if rect_w == 0 || rect_h == 0 || rect_x >= surface_width || rect_y >= surface_height {
        return;
    }
    let w = core::cmp::min(rect_w, surface_width - rect_x);
    let h = core::cmp::min(rect_h, surface_height - rect_y);
    let stride = stride_bytes as usize;
    for row in 0..h as usize {
        let row_base = base_va as usize + (rect_y as usize + row) * stride + rect_x as usize * 4;
        for col in 0..w as usize {
            let cell = (row_base + col * 4) as *mut u32;
            // SAFETY: rect is clipped to the surface above.
            unsafe { core::ptr::write_volatile(cell, argb) };
        }
    }
}
