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

use crate::syscall::microkernel::errnos::ERRNO_FAULT;

use super::recv_from_deliver::Outs;
use super::recv_from_drain::drain;
use super::recv_open::{buffer, inbox};

// `MkIpcRecvFrom`. Same drain semantics as `MkIpcRecv`, with an
// extra `sender_pid_out` user pointer written with the caller pid
// of the dequeued message (0 for kernel-internal senders) so the
// receiver can reply via `MkIpcSendToPid`, and `enqueued_ms_out`
// written with when the kernel queued it, in MkTimeMonotonic ms.
pub fn sys_ipc_recv_from(
    endpoint: u64,
    buf: u64,
    len: usize,
    timeout_ms: u64,
    sender_pid_out: u64,
    enqueued_ms_out: u64,
) -> i64 {
    if let Err(e) = buffer(buf, len) {
        return e;
    }
    for (at, size) in [(sender_pid_out, 4), (enqueued_ms_out, 8)] {
        if at != 0 && crate::usercopy::validate_user_write(at, size).is_err() {
            return ERRNO_FAULT;
        }
    }
    match inbox(endpoint) {
        Ok(name) => {
            drain((buf, len), timeout_ms, Outs::From(sender_pid_out, enqueued_ms_out), &name)
        }
        Err(e) => e,
    }
}
