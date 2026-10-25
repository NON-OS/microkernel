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

pub fn sys_ipc_reply(dest_pid: u64, buf: u64, len: usize) -> i64 {
    if len == 0
        || len > crate::ipc::channel::MAX_MESSAGE_SIZE
        || dest_pid == 0
        || dest_pid > u32::MAX as u64
    {
        return ERRNO_INVAL;
    }
    let data = match copy_in(buf, len) {
        Ok(data) => data,
        Err(e) => return e,
    };
    let caller_pid = current_pid().unwrap_or(0);
    let dest = reply_inbox::for_pid(dest_pid as u32);
    // The token comes from the pending entry that authorizes this reply: the
    // request this server dequeued last when it is this caller's, else the
    // caller's oldest (pending_reply/reply_rule.rs).
    let Some(token) = pending_reply::take_for_reply(caller_pid, &dest) else {
        return 0;
    };
    let from = alloc::format!("proc.{}", caller_pid);
    let mut msg = match IpcMessage::new(&from, &dest, &data) {
        Ok(msg) => msg,
        Err(_) => return ERRNO_NOMEM,
    };
    msg.correlation = token;
    deliver(caller_pid, dest_pid, &dest, msg)
}
