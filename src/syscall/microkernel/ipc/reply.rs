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
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOMEM};

use super::payload_in::copy_in;
use super::pending_reply;
use super::reply_deliver::deliver;
use super::reply_inbox;
use super::reply_trace::trace;

pub fn sys_ipc_reply(dest_pid: u64, buf: u64, len: usize) -> i64 {
    if len == 0
        || len > crate::ipc::channel::MAX_MESSAGE_SIZE
        || dest_pid == 0
        || dest_pid > u32::MAX as u64
    {
        trace(current_pid().unwrap_or(0), dest_pid, b"inval", ERRNO_INVAL, "");
        return ERRNO_INVAL;
    }
    let data = match copy_in(buf, len) {
        Ok(data) => data,
        Err(e) => {
            trace(current_pid().unwrap_or(0), dest_pid, b"copy", e, "");
            return e;
        }
    };
    let caller_pid = current_pid().unwrap_or(0);
    let dest = reply_inbox::for_pid(dest_pid as u32);
    // The token comes from the same pending-reply entry that authorizes this
    // reply, matched by the caller's inbox. Using that entry's token (rather
    // than a separate per-client FIFO) keeps the stamp correct when several
    // callers have replies outstanding on this server at once.
    let Some(token) = pending_reply::remove(caller_pid, &dest) else {
        trace(caller_pid, dest_pid, b"drop no-call", 0, &dest);
        return 0;
    };
    let from = alloc::format!("proc.{}", caller_pid);
    let mut msg = match IpcMessage::new(&from, &dest, &data) {
        Ok(msg) => msg,
        Err(_) => {
            trace(caller_pid, dest_pid, b"message", ERRNO_NOMEM, &dest);
            return ERRNO_NOMEM;
        }
    };
    msg.correlation = token;
    deliver(caller_pid, dest_pid, &dest, msg)
}
