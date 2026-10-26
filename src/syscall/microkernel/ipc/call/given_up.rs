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

//! A call's wait for its reply, and what a wait that runs out tells the
//! server: the call gives up its entry (pending_reply/give_up_rule.rs), so
//! an MkIpcReplyTo that comes later returns ETIMEDOUT rather than 0 for an
//! answer no one reads. An answer already on its way is waited for and
//! taken, so the server's 0 and the caller's answer always go together.

use crate::syscall::microkernel::errnos::ERRNO_TIMEDOUT;

use super::super::pending_reply::{self, GiveUp};
use super::super::recv_reply::recv_reply_correlated;

/// The reply to the call `token` on `server` (None: no server known),
/// into `(resp, len)` within `timeout_ms`; ETIMEDOUT once given up.
pub(super) fn receive(
    (server, pid, token): (Option<u32>, u32, u64),
    inbox: &str,
    (resp, len): (u64, usize),
    timeout_ms: u64,
) -> i64 {
    let rc = recv_reply_correlated(pid, inbox, resp, len, timeout_ms, token);
    let Some(server) = server.filter(|_| rc == ERRNO_TIMEDOUT) else {
        return rc;
    };
    loop {
        match pending_reply::give_up(server, token) {
            GiveUp::Marked => return rc,
            // The server holds the entry while its answer is enqueued.
            GiveUp::Delivering => crate::sched::yield_now(),
            // Whatever came is queued already: look once, 1 ms.
            GiveUp::Gone => return recv_reply_correlated(pid, inbox, resp, len, 1, token),
        }
    }
}
