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

use super::constants::{CQ_BYTES, IO_ENTRIES};
use super::queue::IoQueue;
use crate::admin::CqCursor;

impl IoQueue {
    /// Start the queue over in the memory it has, once a disable has dropped
    /// every command the controller held: no command is out, the tail and
    /// cursor are back at slot 0, and the completion ring is zeroed so no
    /// entry from an earlier pass passes for new. Command ids carry on.
    pub(crate) fn restart(&mut self) {
        // SAFETY: the ring is this queue's own mapped completion region of
        // CQ_BYTES, and the disabled controller does not write it.
        unsafe { core::ptr::write_bytes(self.cq.user_va() as *mut u8, 0, CQ_BYTES as usize) };
        // The zeroed ring is in memory before the queue is created again.
        nonos_libc::dma_wmb();
        self.sq_tail = 0;
        self.cursor = CqCursor::new(IO_ENTRIES);
        self.out = None;
    }
}
