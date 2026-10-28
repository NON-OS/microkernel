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

//! A claimed frame, or a death notice, handed over to the supervisor.

use super::super::frame::{ForeignFrame, NR_DIED};
use super::super::{notice, trap_claim};
use crate::syscall::microkernel::errnos::ERRNO_FAULT;
use crate::usercopy::write_user_value;

// A claimed guest call if one is waiting, else a one-shot death notice built
// into a frame the supervisor recognises by its nr.
pub(super) fn next_delivery(caller: u32) -> Option<ForeignFrame> {
    if let Some(frame) = trap_claim::claim_next(caller) {
        return Some(frame);
    }
    let (pid, code) = notice::take(caller)?;
    Some(ForeignFrame::new(pid, NR_DIED, [code as u64, 0, 0, 0, 0, 0], 0))
}

/// Hand one claimed frame over, or give it back when the supervisor's buffer
/// will not take it.
// A death notice is not in the parked table, so a failed write cannot be
// recovered by unclaiming it; re-post it so the death is not lost to a hang.
pub(super) fn deliver_or_repost(
    caller: u32,
    out_ptr: u64,
    frame: ForeignFrame,
    size: usize,
) -> i64 {
    let rc = deliver(out_ptr, frame, size);
    if rc < 0 && frame.nr == NR_DIED {
        notice::post(caller, frame.pid, frame.arg0 as i32);
    }
    rc
}

fn deliver(out_ptr: u64, frame: ForeignFrame, size: usize) -> i64 {
    if write_user_value(out_ptr, &frame).is_err() {
        trap_claim::unclaim(frame.pid);
        return ERRNO_FAULT;
    }
    size as i64
}
