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

//! A freshly set mode shows whatever VRAM held; the driver paints it once.

use core::ptr::write_volatile;

/// # Safety
///
/// `fb_user_va` is a live mapping of at least `pixels * 4` bytes.
pub unsafe fn clear(fb_user_va: u64, pixels: u64, color: u32) {
    let base = fb_user_va as *mut u32;
    for i in 0..pixels as usize {
        // SAFETY: `i < pixels`, inside the mapping the caller vouches for.
        unsafe { write_volatile(base.add(i), color) };
    }
}
