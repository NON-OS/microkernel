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

use crate::ipc::nonos_inbox;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT};

use super::inbox_name::resolve_for_recv;
use super::recv_from_drain::drain;

// `MkIpcRecvFrom`. Same drain semantics as `MkIpcRecv`, with an
// extra `sender_pid_out` user pointer written with the caller pid
// of the dequeued message (0 for kernel-internal senders) so the
// receiver can reply via `MkIpcSendToPid`, and `enqueued_ms_out`
// written with when the kernel queued it, in MkTimeMonotonic ms (K14).
pub fn sys_ipc_recv_from(
    endpoint: u64,
    buf: u64,
    len: usize,
    timeout_ms: u64,
    sender_pid_out: u64,
    enqueued_ms_out: u64,
) -> i64 {
    if len == 0 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(buf, len).is_err() {
        return ERRNO_FAULT;
    }
    if sender_pid_out != 0
        && crate::usercopy::validate_user_write(sender_pid_out, core::mem::size_of::<u32>())
            .is_err()
    {
        return ERRNO_FAULT;
    }
    if enqueued_ms_out != 0 && crate::usercopy::validate_user_write(enqueued_ms_out, 8).is_err() {
        return ERRNO_FAULT;
    }
    let pid = current_pid().unwrap_or(0);
    let inbox_name = match resolve_for_recv(endpoint, pid) {
        Ok(name) => name,
        Err(e) => return e,
    };
    if !nonos_inbox::exists(&inbox_name) {
        return ERRNO_NOENT;
    }
    drain(buf, len, timeout_ms, (sender_pid_out, enqueued_ms_out), &inbox_name)
}
