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

use super::constants::{ADMIN_ENTRIES, ADMIN_SQ_ID, CQ_BYTES};
use super::cq0_head::cq0_head;
use super::types::AdminQueue;
use crate::admin::{wait_for_completion, Completion};
use crate::clock;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

// The cursor keeps the head below ADMIN_ENTRIES, so every slot the wait reads
// lies inside the completion queue's DMA region.
const _: () = assert!(ADMIN_ENTRIES as u64 * core::mem::size_of::<Completion>() as u64 <= CQ_BYTES);

impl AdminQueue {
    /// `wait` with a budget of `ms` on the clock.
    pub(super) fn wait_ms(
        &mut self,
        regs: Regs,
        stride: u8,
        cid: u16,
        what: &str,
        ms: u64,
    ) -> NvmeResult<()> {
        let budget = clock::budget(ms);
        let mut clock_lost = false;
        let ring = self.cq.user_va();
        let done = wait_for_completion(
            &mut self.cursor,
            ADMIN_SQ_ID,
            cid,
            |head| {
                let slot = ring + (head as u64) * (core::mem::size_of::<Completion>() as u64);
                // SAFETY: head is below ADMIN_ENTRIES (the assert above), so
                // the slot lies in the completion queue's mapped DMA region.
                unsafe { read_volatile(slot as *const Completion) }
            },
            // SAFETY: the CQ0 head doorbell lies in the mapped register block.
            |head| unsafe { regs.w32(cq0_head(stride), head as u32) },
            || {
                let now = clock::uptime_ms();
                clock_lost |= now.is_none();
                budget.spent(now)
            },
        );
        let done = match done {
            Err(NvmeError::ControllerTimeout) if clock_lost => Err(NvmeError::ClockFailed),
            other => other,
        };
        if let Err(e) = done {
            self.say_failed(what, e);
        }
        done
    }
}
