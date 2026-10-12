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

//! A capsule answering a mk_ipc_call on its own reply port (send.rs).

extern crate alloc;

use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_PERM};

use super::send_trace::trace;

/*
 * A capsule replying to a request another capsule made with mk_ipc_call:
 * hand the bytes to that caller's private inbox stamped with the token
 * it waits on, and wake it since a reply inbox has no owner the router
 * would wake on its own.
 */
pub(super) fn to_caller(
    pid: u32,
    endpoint: u64,
    data: &[u8],
    (caller_inbox, caller_pid, token): (&str, u32, u64),
) -> i64 {
    if !super::send_caps::caller_satisfies_endpoint(endpoint, caller_inbox)
        || !crate::services::registry::caller_may_reach_pid(caller_pid)
    {
        return ERRNO_PERM;
    }
    trace(pid, endpoint, caller_inbox, data.len());
    /*
     * Enqueue straight into the caller's reply inbox, the exact inbox its
     * blocked mk_ipc_call is draining, stamped with the token that call
     * filters on. Going back through the service resolver would look the
     * reply inbox up as a service and re-resolve it to the caller's proc
     * inbox (the caller adopted this endpoint), where the caller's serve
     * loop eats the reply and the call times out. That is why every
     * userland call into a kernel-reply service (crypto, the block
     * device, vfs) hung: the answer was delivered to the wrong inbox.
     */
    match IpcMessage::new(&alloc::format!("proc.{}", pid), caller_inbox, data) {
        Ok(mut msg) => {
            msg.correlation = token;
            match nonos_inbox::try_enqueue_strict(caller_inbox, msg) {
                Ok(()) => {
                    crate::sched::wake_process(caller_pid);
                    0
                }
                Err(_) => ERRNO_FAULT,
            }
        }
        Err(_) => ERRNO_FAULT,
    }
}
