// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::constants::{
    AVAIL_OFFSET, BUFFER_SIZE, DESC_OFFSET, QUEUE_SIZE, USED_OFFSET, VIRTQ_DESC_F_WRITE,
};
use super::core::RngQueue;
use super::types::{VirtqAvail, VirtqDesc, VirtqUsed};
use core::ptr;
use core::sync::atomic::Ordering;

impl RngQueue {
    #[inline]
    fn desc_ptr(&self) -> *mut VirtqDesc {
        (self.vq_base + DESC_OFFSET) as *mut VirtqDesc
    }
    #[inline]
    fn avail_ptr(&self) -> *mut VirtqAvail {
        (self.vq_base + AVAIL_OFFSET) as *mut VirtqAvail
    }
    #[inline]
    pub(super) fn used_ptr(&self) -> *mut VirtqUsed {
        (self.vq_base + USED_OFFSET) as *mut VirtqUsed
    }

    pub(crate) fn request_random(&mut self, len: usize) -> Result<(), &'static str> {
        let len = len.min(BUFFER_SIZE);
        self.pending_len = len;
        unsafe {
            ptr::write_bytes(self.buf_base as *mut u8, 0, BUFFER_SIZE);
            let desc = &mut *self.desc_ptr();
            desc.addr = crate::arch::paging::kernel_dma(self.buf_base as u64);
            desc.len = len as u32;
            desc.flags = VIRTQ_DESC_F_WRITE;
            desc.next = 0;
            let avail = &mut *self.avail_ptr();
            let idx = self.next_avail_idx;
            avail.ring[(idx % QUEUE_SIZE) as usize] = 0;
            core::sync::atomic::fence(Ordering::SeqCst);
            self.next_avail_idx = self.next_avail_idx.wrapping_add(1);
            avail.idx = self.next_avail_idx;
        }
        self.kick();
        Ok(())
    }
}
