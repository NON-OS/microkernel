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

//! A colour laid over the pixel already on the surface, by its alpha.

/// Lay `r`, `g`, `b` at alpha `a` over the pixel at `cell`.
///
/// # Safety
/// `cell` is a pixel inside a mapped surface.
pub(super) unsafe fn over(cell: *mut u32, r: u32, g: u32, b: u32, a: u32) {
    let out = if a >= 255 {
        (r << 16) | (g << 8) | b
    } else {
        // SAFETY: cell is inside the surface (the caller's promise).
        let d = unsafe { core::ptr::read_volatile(cell) };
        let dr = (d >> 16) & 0xFF;
        let dg = (d >> 8) & 0xFF;
        let db = d & 0xFF;
        let ia = 255 - a;
        (((r * a + dr * ia) / 255) << 16)
            | (((g * a + dg * ia) / 255) << 8)
            | ((b * a + db * ia) / 255)
    };
    // SAFETY: cell is inside the surface (the caller's promise).
    unsafe { core::ptr::write_volatile(cell, 0xFF00_0000 | out) };
}
