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

use crate::process::accounting::{Kind, Total};
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::ERRNO_INVAL;

use super::payload_in::copy_in;
use super::send_address::as_addressed;
use super::send_redirect::{redirect_reply, resolve_send_target, Redirect};
use super::send_reply::to_caller;
use super::send_reply_inbox::to_reply_inbox;

pub fn sys_ipc_send(endpoint: u64, buf: u64, len: usize) -> i64 {
    let rc = send_with_correlation(endpoint, buf, len, 0);
    if rc == 0 {
        crate::process::accounting::bump(current_pid().unwrap_or(0), Kind::IpcTx);
        crate::process::accounting::bump_total(Total::IpcMessages);
    }
    rc
}

pub(super) fn send_with_correlation(endpoint: u64, buf: u64, len: usize, correlation: u64) -> i64 {
    /*
     * Reject oversize before allocating: len is validated against the 64 MiB
     * usercopy ceiling downstream, but the message itself is capped at 1 MiB,
     * so bound it here to avoid a capsule forcing huge transient allocations.
     */
    if len == 0 || len > crate::ipc::channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    let data = match copy_in(buf, len) {
        Ok(data) => data,
        Err(e) => return e,
    };
    let pid = current_pid().unwrap_or(0);
    let target = resolve_send_target(endpoint);
    match redirect_reply(pid, &target) {
        Redirect::ToCaller { caller_inbox, caller_pid, token } => {
            to_caller(pid, &data, (&caller_inbox, caller_pid, token))
        }
        Redirect::ToReplyInbox => to_reply_inbox(pid, &target, &data),
        Redirect::AsAddressed => as_addressed(pid, endpoint, &target, &data, correlation),
    }
}
