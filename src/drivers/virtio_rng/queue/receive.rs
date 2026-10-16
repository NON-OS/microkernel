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

//! Whether the device has filled the buffer, and taking what it wrote.

use super::constants::QUEUE_SIZE;
use super::core::RngQueue;
use core::ptr;

impl RngQueue {
    pub(crate) fn has_completed(&self) -> bool {
        unsafe {
            let used = &*self.used_ptr();
            used.idx != self.last_used_idx
        }
    }

    pub(crate) fn get_received_bytes(&mut self, buf: &mut [u8]) -> usize {
        let received_len = unsafe {
            let used = &*self.used_ptr();
            if used.idx == self.last_used_idx {
                return 0;
            }
            let used_elem = &used.ring[(self.last_used_idx % QUEUE_SIZE) as usize];
            self.last_used_idx = self.last_used_idx.wrapping_add(1);
            used_elem.len as usize
        };
        let copy_len = received_len.min(buf.len()).min(self.pending_len);
        unsafe {
            ptr::copy_nonoverlapping(self.buf_base as *const u8, buf.as_mut_ptr(), copy_len);
        }
        self.pending_len = 0;
        copy_len
    }
}
