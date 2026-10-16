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

extern crate alloc;

use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox::{try_enqueue_strict, StrictEnqueueError};
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_INVAL, ERRNO_NOENT, ERRNO_NOMEM};

use super::payload_in::copy_in;
use super::send_to_pid_trace::trace;

// `MkIpcSendToPid` delivers `buf` to the destination pid's default
// per-process inbox `proc.<pid>`. Used by servers replying to a
// `MkIpcRecvFrom` caller without going through the named-endpoint
// registry. The kernel still records the sender in the message
// envelope so the receiver can chain a follow-up reply.
pub fn sys_ipc_send_to_pid(dest_pid: u64, buf: u64, len: usize) -> i64 {
    if len == 0 || len > crate::ipc::channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    if dest_pid == 0 || dest_pid > u32::MAX as u64 {
        return ERRNO_INVAL;
    }
    let data = match copy_in(buf, len) {
        Ok(data) => data,
        Err(e) => return e,
    };
    let caller_pid = current_pid().unwrap_or(0);
    // A capsule held to a peer list reaches only the inboxes of its peers.
    if !crate::services::registry::caller_may_reach_pid(dest_pid as u32) {
        return crate::syscall::microkernel::errnos::ERRNO_PERM;
    }
    // The inbox is the one every endpoint `dest` serves is read from.
    if !super::send_caps::caller_satisfies_inbox(dest_pid as u32) {
        return crate::syscall::microkernel::errnos::ERRNO_PERM;
    }
    trace(caller_pid, dest_pid, len);
    let dest = alloc::format!("proc.{}", dest_pid as u32);
    let from = alloc::format!("proc.{}", caller_pid);
    let Ok(msg) = IpcMessage::new(&from, &dest, &data) else {
        return ERRNO_NOMEM;
    };
    let rc = match try_enqueue_strict(&dest, msg) {
        Ok(()) => {
            crate::process::accounting::bump(caller_pid, crate::process::accounting::Kind::IpcTx);
            crate::process::accounting::bump_total(crate::process::accounting::Total::IpcMessages);
            0
        }
        Err(StrictEnqueueError::MissingInbox) | Err(StrictEnqueueError::DeadOwner) => ERRNO_NOENT,
        Err(StrictEnqueueError::QueueFull(_)) => ERRNO_BUSY,
    };
    // Wake unconditionally on delivery: wake_process is state-checked and
    // idempotent, while gating on the sleep map strands a receiver that is
    // Sleeping without a map entry, where no sender could ever revive it.
    if rc == 0 {
        crate::sched::wake_process(dest_pid as u32);
    }
    rc
}
