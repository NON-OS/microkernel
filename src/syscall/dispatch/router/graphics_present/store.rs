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

//! The framebuffer stores of a present.

/// Store `bytes` at `dst_ptr`, a whole pixel at a time. The framebuffer is
/// write-combining, so the store count is the cost and byte-wise was four
/// times as many. Falls back to bytes if a firmware stride leaves the
/// destination unaligned.
///
/// # Safety
///
/// `dst_ptr..dst_ptr + bytes.len()` must lie inside the mapped framebuffer.
pub(super) unsafe fn store(dst_ptr: *mut u8, bytes: &[u8]) {
    if (dst_ptr as usize) % 4 == 0 && bytes.len() % 4 == 0 {
        for (i, px) in bytes.chunks_exact(4).enumerate() {
            let word = u32::from_ne_bytes([px[0], px[1], px[2], px[3]]);
            /*
             * SAFETY: `dst_ptr` is 4-byte aligned on this branch and
             * `i < bytes.len() / 4`, so the write is in bounds and aligned.
             */
            unsafe { core::ptr::write_volatile((dst_ptr as *mut u32).add(i), word) };
        }
    } else {
        for (i, &b) in bytes.iter().enumerate() {
            /*
             * SAFETY: `i < bytes.len()`, inside the caller's range.
             */
            unsafe { core::ptr::write_volatile(dst_ptr.add(i), b) };
        }
    }
}
