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

//! The I/O queue's completion wait: the loop both queues share, run over
//! this queue's ring until `expired` (park::parked) says the time is up.

use super::constants::{CQ_BYTES, IO_ENTRIES, IO_QID};
use crate::admin::{read_slot, wait_for_completion, Completion, CqCursor};
use crate::error::NvmeResult;
use crate::regs::Regs;

// The cursor keeps the head below IO_ENTRIES, so every slot the wait reads
// lies inside the completion queue's DMA region.
const _: () = assert!(IO_ENTRIES as u64 * core::mem::size_of::<Completion>() as u64 <= CQ_BYTES);

/// Wait for command `cid` on the completion ring at `ring`, whose head
/// doorbell is `doorbell`. A wait that `expired` ends is given up on.
pub(super) fn ring_wait<X: FnMut() -> bool>(
    cursor: &mut CqCursor,
    ring: u64,
    doorbell: u32,
    regs: Regs,
    cid: u16,
    expired: X,
) -> NvmeResult<()> {
    wait_for_completion(
        cursor,
        IO_QID,
        cid,
        |head| {
            let slot = ring + (head as u64) * (core::mem::size_of::<Completion>() as u64);
            // SAFETY: head is below IO_ENTRIES (the assert above), so the
            // slot lies in the completion queue's mapped DMA region.
            unsafe { read_slot(slot) }
        },
        // SAFETY: the queue's head doorbell lies in the register block,
        // which bring-up checked holds the doorbells of queue 1.
        |head| unsafe { regs.w32(doorbell, head as u32) },
        expired,
    )
}
