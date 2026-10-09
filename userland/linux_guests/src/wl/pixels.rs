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

//! The window's pixels: a memfd sized, mapped shared and painted.

use super::draw::paint;
use crate::sys::{call, MMAP, PROT_RW};

// A memfd sized and mapped shared, painted, and handed back as the fd.
pub fn pixels(w: u32, h: u32) -> Option<i32> {
    let fd = call(319, [b"window\0".as_ptr() as u64, 0, 0, 0, 0, 0]);
    let size = (w * h * 4) as u64;
    if fd < 0 || call(77, [fd as u64, size, 0, 0, 0, 0]) != 0 {
        return None;
    }
    let at = call(MMAP, [0, size, PROT_RW, 0x01, fd as u64, 0]);
    if at < 0 {
        return None;
    }
    // SAFETY: `at` is a fresh shared mapping of `size` bytes, u32-aligned.
    let px = unsafe { core::slice::from_raw_parts_mut(at as *mut u32, (w * h) as usize) };
    paint(px, w as usize, h as usize);
    Some(fd as i32)
}
