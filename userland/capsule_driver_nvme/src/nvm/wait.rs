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

use core::ptr::read_volatile;

use super::constants::{COMPLETION_TIMEOUT_MS, CQ_BYTES, IO_ENTRIES, IO_QID};
use super::queue::IoQueue;
use crate::admin::{wait_noting_foreign, Completion};
use crate::clock;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

// The cursor keeps the head below IO_ENTRIES, so every slot the wait reads
// lies inside the completion queue's DMA region.
const _: () = assert!(IO_ENTRIES as u64 * core::mem::size_of::<Completion>() as u64 <= CQ_BYTES);

impl IoQueue {
    /// Wait for `cid`. Should its wait run out, the command is remembered as
    /// still the controller's.
    pub(super) fn wait(&mut self, regs: Regs, cid: u16) -> NvmeResult<()> {
        let done = self.wait_for(regs, cid);
        if matches!(done, Err(NvmeError::ControllerTimeout)) {
            self.out = Some(cid);
        }
        done
    }

    /*
     * A command given up on is waited out before the data buffer is filled
     * or another command goes in: a write still in flight could otherwise
     * take the next write's bytes to its own sectors, and a read still in
     * flight could land in the buffer under the next read's answer. Refused,
     * buffer untouched, while it stays out.
     */
    pub fn settle(&mut self, regs: Regs) -> NvmeResult<()> {
        let Some(cid) = self.out else { return Ok(()) };
        self.wait_for(regs, cid)?;
        self.out = None;
        Ok(())
    }

    fn wait_for(&mut self, regs: Regs, cid: u16) -> NvmeResult<()> {
        // A clock that cannot be read spends the budget at once: the command
        // is then given up on like any that timed out, and stays out until
        // its completion is seen.
        let budget = clock::budget(COMPLETION_TIMEOUT_MS);
        let ring = self.cq.user_va();
        let doorbell = self.cq_db;
        let out = &mut self.out;
        wait_noting_foreign(
            &mut self.cursor,
            IO_QID,
            cid,
            |head| {
                let slot = ring + (head as u64) * (core::mem::size_of::<Completion>() as u64);
                unsafe { read_volatile(slot as *const Completion) }
            },
            |head| unsafe { regs.w32(doorbell, head as u32) },
            || budget.spent(clock::uptime_ms()),
            |seen| {
                if *out == Some(seen) {
                    *out = None;
                }
            },
        )
    }
}
