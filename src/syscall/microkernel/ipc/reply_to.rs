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
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_NOMEM, ERRNO_TIMEDOUT};

use super::payload_in::copy_in;
use super::pending_reply::{self, Named};
use super::reply_deliver::deliver;

/// `MkIpcReplyTo`: answer the request MkIpcRecvReq named `request`, and
/// only that one. EINVAL for request 0 (nothing owed) or a bad length;
/// EFAULT for an unreadable buffer, with the request still owed; ENOENT
/// when the server has no such request dequeued and unanswered: never
/// queued here, still in the inbox, or already answered; ETIMEDOUT when
/// its call stopped waiting, so no one has the answer. 0 only when the
/// caller has it (pending_reply/give_up_rule.rs). The request is spent
/// once found, as delivery's own errors (EBUSY, ENOENT) also say.
pub fn sys_ipc_reply_to(request: u64, buf: u64, len: usize) -> i64 {
    if request == 0 || len == 0 || len > crate::ipc::channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    let data = match copy_in(buf, len) {
        Ok(data) => data,
        Err(e) => return e,
    };
    let server = current_pid().unwrap_or(0);
    let (caller, inbox, token) = match pending_reply::take_named(server, request) {
        Some(Named::Owed(caller, inbox, token)) => (caller, inbox, token),
        Some(Named::GivenUp) => return ERRNO_TIMEDOUT,
        None => return ERRNO_NOENT,
    };
    let from = alloc::format!("proc.{}", server);
    let rc = match IpcMessage::new(&from, &inbox, &data) {
        Ok(mut msg) => {
            msg.correlation = token;
            deliver(server, caller as u64, &inbox, msg)
        }
        Err(_) => ERRNO_NOMEM,
    };
    // Delivered or not, the entry goes; a caller waiting on it looks once more.
    pending_reply::delivered(server, request);
    rc
}
