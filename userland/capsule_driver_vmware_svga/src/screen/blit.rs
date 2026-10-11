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

//! Copying a rectangle from the compositor's surface into the framebuffer.
//! The two have their own strides: the surface is packed, the framebuffer
//! line is whatever BYTES_PER_LINE says.

use crate::constants::mode::BYTES_PER_PIXEL;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Plane {
    pub va: u64,
    pub stride: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Whether `r` is a non-empty rectangle inside a `width` by `height` plane.
pub const fn rect_fits(r: Rect, width: u32, height: u32) -> bool {
    let x_end = r.x.checked_add(r.w);
    let y_end = r.y.checked_add(r.h);
    match (x_end, y_end) {
        (Some(xe), Some(ye)) => r.w > 0 && r.h > 0 && xe <= width && ye <= height,
        _ => false,
    }
}

/// Copy `r` from `src` to the same place in `dst`.
///
/// # Safety
/// Both planes are live mappings of at least `stride * height` bytes, and
/// `r` fits both (`rect_fits`).
pub unsafe fn copy_rect(src: &Plane, dst: &Plane, r: Rect) {
    let row_bytes = (r.w * BYTES_PER_PIXEL) as usize;
    let col = (r.x * BYTES_PER_PIXEL) as u64;
    for row in r.y..r.y + r.h {
        let s = src.va + row as u64 * src.stride as u64 + col;
        let d = dst.va + row as u64 * dst.stride as u64 + col;
        core::ptr::copy_nonoverlapping(s as *const u8, d as *mut u8, row_bytes);
    }
}
