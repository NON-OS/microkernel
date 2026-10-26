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

use super::recv_meta::meta_v1;
use super::sender_pid::from_envelope;

/// Where a receive writes what it learns beside the bytes.
#[derive(Clone, Copy)]
pub(super) enum Outs {
    /// MkIpcRecvFrom: the sender's pid and when it was queued, each to its
    /// pointer when that is non-null.
    From(u64, u64),
    /// MkIpcRecvReq: the v1 meta (recv_meta.rs), to a pointer checked for
    /// its whole length.
    Meta(u64),
}

/// The dequeued `msg` handed to `pid`: its bytes to `buf`, then `outs`.
/// `request` is the server's id for it, 0 when no reply is owed.
pub(super) fn deliver(
    pid: u32,
    msg: &crate::ipc::nonos_channel::IpcMessage,
    (buf, len): (u64, usize),
    outs: Outs,
    request: u64,
) -> i64 {
    let copy_len = msg.data.len().min(len);
    if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
        return ERRNO_FAULT;
    }
    let enqueued = || {
        let now = (crate::sys::clock::since_boot_ms(), crate::time::timestamp_millis());
        super::enqueued_at::enqueued_uptime(now.0, now.1, msg.timestamp_ms)
    };
    let sender = from_envelope(&msg.from);
    let written = match outs {
        Outs::From(sender_out, enqueued_out) => put(sender_out, &sender.to_le_bytes())
            .and_then(|()| put(enqueued_out, &enqueued().to_le_bytes())),
        Outs::Meta(at) => {
            crate::usercopy::copy_to_user(at, &meta_v1(sender, enqueued(), request)).map_err(|_| ())
        }
    };
    if written.is_err() {
        return ERRNO_FAULT;
    }
    crate::process::accounting::bump(pid, crate::process::accounting::Kind::IpcRx);
    copy_len as i64
}

/// `bytes` to the user pointer `at`; a null one, which MkIpcRecvFrom
/// allows, is skipped.
fn put(at: u64, bytes: &[u8]) -> Result<(), ()> {
    if at == 0 {
        return Ok(());
    }
    crate::usercopy::copy_to_user(at, bytes).map_err(|_| ())
}
