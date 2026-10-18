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

use super::constants::{ADMIN_ENTRIES, CQ_BYTES};
use super::types::AdminQueue;
use crate::admin::CqCursor;
use crate::regs::Regs;

impl AdminQueue {
    /// Start the admin queue over in the memory it has, once the controller
    /// is disabled and so reads neither ring. The completion ring is zeroed:
    /// an entry left from an earlier pass can carry the phase tag the fresh
    /// cursor waits for, and would pass for a new completion. Command ids
    /// carry on; the disable retired every one the controller held.
    pub fn restart(&mut self, regs: Regs) {
        // SAFETY: the ring is this queue's own mapped completion region of
        // CQ_BYTES, and the disabled controller does not write it.
        unsafe { core::ptr::write_bytes(self.cq.user_va() as *mut u8, 0, CQ_BYTES as usize) };
        // The zeroed ring is in memory before the controller is told of it.
        nonos_libc::dma_wmb();
        self.tail = 0;
        self.cursor = CqCursor::new(ADMIN_ENTRIES);
        self.program_registers(regs);
    }
}
