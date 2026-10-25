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

use super::completion_step::{classify, CqStep};
use super::cq_cursor::CqCursor;
use crate::admin::Completion;
use crate::error::{NvmeError, NvmeResult};

/// The deadline is checked only every this many spins, so the completion
/// poll stays a tight loop and does not make a syscall per iteration.
pub const DEADLINE_CHECK_SPINS: u32 = 1024;

/// Poll the completion queue at `cursor` until the entry for command `cid`
/// on submission queue `sq_id` arrives or `expired` says the time is up. The
/// admin and I/O queues share this loop; each passes its own slot read (a
/// volatile read of the DMA ring at a head the cursor keeps below
/// `entries`), its own head doorbell write and its own deadline, so the host
/// proofs drive the same loop with a scripted ring and a scripted clock.
pub fn wait_for_completion<R, D, X>(
    cursor: &mut CqCursor,
    sq_id: u16,
    cid: u16,
    mut read_slot: R,
    mut ring_head: D,
    mut expired: X,
) -> NvmeResult<()>
where
    R: FnMut(u16) -> Completion,
    D: FnMut(u16),
    X: FnMut() -> bool,
{
    let (mut spins, mut last) = (0u32, false);
    loop {
        let entry = read_slot(cursor.head);
        match classify(entry, cursor.phase, sq_id, cid) {
            CqStep::Done(result) => {
                cursor.advance();
                ring_head(cursor.head);
                return result;
            }
            // Not this command's, but the controller wrote it this pass, so
            // it is consumed and the wait goes on. Left in place it would sit
            // at the head for good: a completion that arrives after its own
            // wait timed out would fail every later command on the queue.
            CqStep::Foreign => {
                cursor.advance();
                ring_head(cursor.head);
            }
            CqStep::Empty => {}
        }
        // The deadline can pass while the wait is parked or not running, with
        // the completion written meanwhile: the slot is read once more after
        // it, and only then is the wait given up.
        if last {
            return Err(NvmeError::ControllerTimeout);
        }
        spins = spins.wrapping_add(1);
        last = last || (spins.is_multiple_of(DEADLINE_CHECK_SPINS) && expired());
        core::hint::spin_loop();
    }
}
