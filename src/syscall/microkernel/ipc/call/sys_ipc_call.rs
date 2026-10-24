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

use crate::process::{accounting, current_pid};
use crate::services::registry::lookup_port;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL};
use crate::syscall::microkernel::narrow::u32_arg;

use super::super::pending_reply;
use super::super::recv::recv_reply_correlated;
use super::super::reply_inbox;
use super::super::send::send_with_correlation;
use super::settle::settle;
use super::token::next_call_token;
use super::trace::trace;

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
    // Registered with the pending reply keyed by this caller's inbox (token.rs).
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
    trace(pid, b"send", send_result);
    if send_result < 0 {
        if let Some(server_pid) = endpoint_pid {
            // Strip exactly the entry this call pushed: the newest match. An
            // older match can be a reply still owed from a timed-out call, and
            // removing it shifts the server's FIFO onto the wrong callers.
            pending_reply::remove_latest(server_pid, &inbox);
        }
        return send_result;
    }
    accounting::bump(pid, accounting::Kind::IpcTx);
    accounting::bump_total(accounting::Total::IpcMessages);
    let timeout = if timeout_ms == 0 { 5000 } else { timeout_ms };
    let recv_result = recv_reply_correlated(pid, &inbox, resp, resp_len, timeout, token);
    settle(endpoint.as_ref(), pid, (req, req_len, resp), recv_result);
    trace(pid, b"recv", recv_result);
    recv_result
}
