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

//! A request's data through the data buffer: copied in before a write,
//! out after a read, and wiped once it is delivered (C8), so the buffer
//! holds no card data between requests. The fences keep the copies on
//! their side of the register writes that start a transfer and the reads
//! that see it end.

use core::sync::atomic::{fence, Ordering};

use super::region::Region;

impl Region {
    /// `data` to the start of the buffer; false, and nothing copied, when
    /// it does not fit.
    pub fn copy_in(&self, data: &[u8]) -> bool {
        if data.len() as u64 > self.len {
            return false;
        }
        // SAFETY: `data.len()` bytes from `user_va` lie inside the mapping
        // (checked against `len`), which lives until the claim is released;
        // the source is the caller's own memory, not the mapping.
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.user_va as *mut u8, data.len())
        };
        fence(Ordering::Release);
        true
    }

    /// The start of the buffer into `out`; false, and nothing copied, when
    /// `out` is longer than the buffer.
    pub fn copy_out(&self, out: &mut [u8]) -> bool {
        if out.len() as u64 > self.len {
            return false;
        }
        fence(Ordering::Acquire);
        // SAFETY: as copy_in, with the mapping the source.
        unsafe {
            core::ptr::copy_nonoverlapping(self.user_va as *const u8, out.as_mut_ptr(), out.len())
        };
        true
    }

    /// Zero the first `len` bytes, or all of it.
    pub fn wipe(&self, len: usize) {
        let n = (len as u64).min(self.len) as usize;
        // SAFETY: `n` bytes from `user_va`, at most `len`, inside the mapping.
        unsafe { core::ptr::write_bytes(self.user_va as *mut u8, 0, n) };
    }
}
