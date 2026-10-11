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

//! A present: the damaged rect, from the shadow into VRAM, a pixel at a time.

use core::ptr::{read_volatile, write_volatile};

use super::screen::Rect;

/// # Safety
///
/// `shadow` and `vram` each map the whole mode (`stride * height` bytes),
/// and `r` and `offset` passed `Screen::inside` and `Screen::source`.
pub unsafe fn copy(shadow: u64, vram: u64, stride: u32, r: Rect, offset: u64) {
    let stride = stride as u64;
    for row in 0..r.h as u64 {
        let src = (shadow + offset + row * stride) as *const u32;
        let dst = (vram + (r.y as u64 + row) * stride + r.x as u64 * 4) as *mut u32;
        for i in 0..r.w as usize {
            // SAFETY: the caller's checks put both rows inside their maps;
            // the compositor writes the shadow from another process, so it
            // is read volatile, and VRAM is device memory.
            unsafe { write_volatile(dst.add(i), read_volatile(src.add(i))) };
        }
    }
}
