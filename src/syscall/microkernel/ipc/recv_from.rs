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
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_TIMEDOUT};

use super::inbox_name::resolve_for_recv;
use super::sender_pid::from_envelope;

// `MkIpcRecvFrom`. Same drain semantics as `MkIpcRecv`, with an
// extra `sender_pid_out` user pointer written with the caller pid
// of the dequeued message (0 for kernel-internal senders) so the
// receiver can reply via `MkIpcSendToPid`.
pub fn sys_ipc_recv_from(
    endpoint: u64,
    buf: u64,
    len: usize,
    timeout_ms: u64,
    sender_pid_out: u64,
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
    let pid = current_pid().unwrap_or(0);
    let inbox_name = match resolve_for_recv(endpoint, pid) {
        Ok(name) => name,
        Err(e) => return e,
    };
    if !nonos_inbox::exists(&inbox_name) {
        return ERRNO_NOENT;
    }
    drain(buf, len, timeout_ms, sender_pid_out, &inbox_name)
}

fn deliver(
    pid: u32,
    msg: &crate::ipc::nonos_channel::IpcMessage,
    buf: u64,
    len: usize,
    sender_pid_out: u64,
) -> i64 {
    let sender_pid = from_envelope(&msg.from);
    let copy_len = msg.data.len().min(len);
    if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
        return ERRNO_FAULT;
    }
    if sender_pid_out != 0 {
        let bytes = sender_pid.to_le_bytes();
        if crate::usercopy::copy_to_user(sender_pid_out, &bytes).is_err() {
            return ERRNO_FAULT;
        }
    }
    crate::process::accounting::bump(pid, crate::process::accounting::Kind::IpcRx);
    copy_len as i64
}

fn drain(buf: u64, len: usize, timeout_ms: u64, sender_pid_out: u64, inbox: &str) -> i64 {
    let start = crate::time::timestamp_millis();
    let pid = current_pid().unwrap_or(0);
    loop {
        // Token before the check, so a delivery landing between an empty
        // check and the transition to Sleeping blocks the sleep instead of
        // spending its wake on a still-Running process. Same lost-wakeup
        // window as recv_from_inbox; this loop is the one most servers run.
        let token = crate::sched::wake_token(pid);
        if let Some(msg) = nonos_inbox::try_dequeue_existing(inbox) {
            super::pending_reply::record_served(pid, msg.correlation);
            return deliver(pid, &msg, buf, len, sender_pid_out);
        }
        let elapsed = crate::time::timestamp_millis().saturating_sub(start);
        if timeout_ms > 0 && elapsed >= timeout_ms {
            return ERRNO_TIMEDOUT;
        }
        let deadline = if timeout_ms == 0 { u64::MAX } else { start.saturating_add(timeout_ms) };
        crate::sched::sleep_until_unless_woken(pid, deadline, token);
        if let Some(msg) = nonos_inbox::try_dequeue_existing(inbox) {
            super::pending_reply::record_served(pid, msg.correlation);
            return deliver(pid, &msg, buf, len, sender_pid_out);
        }
        crate::sched::yield_now();
    }
}
