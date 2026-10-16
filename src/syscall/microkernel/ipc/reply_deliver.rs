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

//! MkIpcReply's last step: the reply onto its caller's inbox, the caller
//! woken.

use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox::{try_enqueue_strict, StrictEnqueueError};
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_NOENT};

use super::reply_trace::trace;

pub(super) fn deliver(caller_pid: u32, dest_pid: u64, dest: &str, msg: IpcMessage) -> i64 {
    let rc = match try_enqueue_strict(dest, msg) {
        Ok(()) => {
            crate::sched::wake_process(dest_pid as u32);
            crate::process::accounting::bump(caller_pid, crate::process::accounting::Kind::IpcTx);
            crate::process::accounting::bump_total(crate::process::accounting::Total::IpcMessages);
            0
        }
        Err(StrictEnqueueError::MissingInbox) | Err(StrictEnqueueError::DeadOwner) => ERRNO_NOENT,
        Err(StrictEnqueueError::QueueFull(_)) => ERRNO_BUSY,
    };
    // Wake unconditionally on delivery (state-checked and idempotent); the
    // sleep-map gate strands a receiver that is Sleeping without an entry.
    let mut woke = false;
    if rc == 0 {
        crate::sched::wake_process(dest_pid as u32);
        woke = true;
    }
    trace(caller_pid, dest_pid, if woke { b"enqueue wake=1" } else { b"enqueue wake=0" }, rc, dest);
    rc
}
