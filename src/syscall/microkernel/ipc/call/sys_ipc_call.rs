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

use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL};
use crate::syscall::microkernel::narrow::u32_arg;
use crate::{process::current_pid, services::registry::lookup_port};

use super::super::pending_reply;
use super::super::reply_inbox;
use super::super::send::send_with_correlation;
use super::answer::answer;
use super::gpu_bench::mark_first;
use super::token::next_call_token;

pub fn sys_ipc_call(
    ep: u64,
    req: u64,
    req_len: usize,
    resp: u64,
    resp_len: usize,
    timeout_ms: u64,
) -> i64 {
    let pid = current_pid().unwrap_or(0);
    if resp_len == 0 || resp_len > crate::ipc::channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(resp, resp_len).is_err() {
        return ERRNO_FAULT;
    }
    let token = next_call_token();
    let inbox = reply_inbox::for_pid(pid);
    let endpoint = u32_arg(ep).and_then(lookup_port);
    let endpoint_pid = endpoint.as_ref().map(|endpoint| endpoint.pid);
    if let Some(server_pid) = endpoint_pid {
        if !pending_reply::push(server_pid, pid, inbox.clone(), token) {
            return ERRNO_BUSY;
        }
    }
    let send_result = send_with_correlation(ep, req, req_len, token);
    if send_result < 0 {
        if let Some(server_pid) = endpoint_pid {
            // Strip exactly the entry this call pushed: the newest match. An
            // older match can be a reply still owed from a timed-out call, and
            // removing it shifts the server's FIFO onto the wrong callers.
            pending_reply::remove_latest(server_pid, &inbox);
        }
        return send_result;
    }
    crate::process::accounting::bump(pid, crate::process::accounting::Kind::IpcTx);
    crate::process::accounting::bump_total(crate::process::accounting::Total::IpcMessages);
    let recv_result = answer(pid, &inbox, endpoint.as_ref(), resp, resp_len, timeout_ms, token);
    if let Some(endpoint) = endpoint.as_ref() {
        mark_first(&endpoint.name, req, req_len, resp, recv_result);
    }
    recv_result
}
