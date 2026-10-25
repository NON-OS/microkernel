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
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_NOENT, ERRNO_SRCH, ERRNO_TIMEDOUT};

/// Receive the reply to an outstanding `mk_ipc_call`, accepting ONLY a message
/// whose correlation equals the token the call sent. Every real reply carries
/// that token, because `mk_ipc_reply` stamps it from the correlation the server
/// captured on the request. A message in the caller's reply inbox with any other
/// correlation is a forgery — a hostile capsule can `mk_ipc_send` into another
/// capsule's reply inbox, but `sys_ipc_send` hardcodes correlation 0, so it can
/// never match a (nonzero) token. Such messages are discarded, not delivered,
/// closing the reply-injection / confused-deputy hole.
pub(super) fn recv_reply_correlated(
    pid: u32,
    inbox_name: &str,
    buf: u64,
    len: usize,
    timeout_ms: u64,
    want: u64,
) -> i64 {
    if !nonos_inbox::exists(&inbox_name) {
        return ERRNO_NOENT;
    }
    let start = crate::time::timestamp_millis();
    loop {
        // Token before the drain, for the same lost-wakeup reason as
        // `recv_from_inbox`: a reply landing between an empty drain and the
        // transition to Sleeping has already spent its wake.
        let token = crate::sched::wake_token(pid);
        // Drain everything queued; deliver the matching reply, drop the rest
        // (forged, stale, or a duplicate from a previous call).
        while let Some(msg) = nonos_inbox::try_dequeue_existing(&inbox_name) {
            if msg.correlation == want {
                let copy_len = msg.data.len().min(len);
                if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
                    return ERRNO_FAULT;
                }
                return copy_len as i64;
            }
            super::call::note_drop(msg.correlation, want);
        }
        // The server died owing this reply (pending_reply::abandoned).
        if super::pending_reply::abandoned(pid, want) {
            return ERRNO_SRCH;
        }
        let elapsed = crate::time::timestamp_millis().saturating_sub(start);
        if timeout_ms > 0 && elapsed >= timeout_ms {
            return ERRNO_TIMEDOUT;
        }
        let deadline = if timeout_ms == 0 { u64::MAX } else { start.saturating_add(timeout_ms) };
        crate::sched::sleep_until_unless_woken(pid, deadline, token);
        crate::sched::yield_now();
    }
}
