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

//! A request for random bytes, and its completion.

use core::sync::atomic::{fence, Ordering};

use super::core::RngQueue;

/// Descriptor flag: the device writes this buffer.
const DESC_F_WRITE: u16 = 2;
const PAGE: usize = 4096;

impl RngQueue {
    /// Hand the device one buffer of `len` bytes (at most a page) to fill,
    /// unless one it has not answered is still out: that one is waited on.
    pub(crate) fn request_random(&mut self, len: usize) {
        if self.pending_len != 0 {
            return;
        }
        let len = len.clamp(1, PAGE);
        self.pending_len = len;
        // Descriptor 0: the buffer, device-writable, alone in its chain.
        self.put64(0, self.buf.phys);
        self.put32(8, len as u32);
        self.put16(12, DESC_F_WRITE);
        self.put16(14, 0);
        let slot = (self.next_avail_idx % self.size) as usize;
        self.put16(self.layout.avail + 4 + 2 * slot, 0);
        // The descriptor and the slot before the index that publishes them.
        fence(Ordering::SeqCst);
        self.next_avail_idx = self.next_avail_idx.wrapping_add(1);
        self.put16(self.layout.avail + 2, self.next_avail_idx);
        fence(Ordering::SeqCst);
        // SAFETY: the probe found this port in the device's own I/O BAR.
        unsafe { crate::sys::io::outw(self.notify_port, 0) };
    }

    pub(crate) fn has_completed(&self) -> bool {
        self.get16(self.layout.used + 2) != self.last_used_idx
    }

    /// Copy what the device wrote into `out`; 0 when nothing completed.
    pub(crate) fn take(&mut self, out: &mut [u8]) -> usize {
        if !self.has_completed() {
            return 0;
        }
        // The used index before the element it publishes.
        fence(Ordering::SeqCst);
        let slot = (self.last_used_idx % self.size) as usize;
        let written = self.get32(self.layout.used + 4 + 8 * slot + 4) as usize;
        self.last_used_idx = self.last_used_idx.wrapping_add(1);
        let n = written.min(out.len()).min(self.pending_len);
        // SAFETY: `buf` is a page of ours and `n` is at most a page.
        unsafe { core::ptr::copy_nonoverlapping(self.buf.virt as *const u8, out.as_mut_ptr(), n) };
        // Nothing of it stays behind for the next reader of the page.
        // SAFETY: as above.
        unsafe { core::ptr::write_bytes(self.buf.virt as *mut u8, 0, n) };
        self.pending_len = 0;
        n
    }
}
