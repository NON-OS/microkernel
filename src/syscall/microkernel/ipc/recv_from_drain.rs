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
use crate::syscall::microkernel::errnos::{ERRNO_AGAIN, ERRNO_TIMEDOUT};

use super::recv_from_deliver::deliver;
use super::wait_rule::{after_empty, Next};

pub(super) fn drain(buf: u64, len: usize, timeout_ms: u64, outs: (u64, u64), inbox: &str) -> i64 {
    let start = crate::time::timestamp_millis();
    let pid = current_pid().unwrap_or(0);
    loop {
        // Token before the check, so a delivery landing between an empty
        // check and the transition to Sleeping blocks the sleep instead of
        // spending its wake on a still-Running process. Same lost-wakeup
        // window as recv_from_inbox; this loop is the one most servers run.
        let token = crate::sched::wake_token(pid);
        if let Some(msg) = nonos_inbox::try_dequeue_existing(inbox) {
            super::pending_reply::mark_served(pid, msg.correlation);
            return deliver(pid, &msg, buf, len, outs);
        }
        let elapsed = crate::time::timestamp_millis().saturating_sub(start);
        let deadline = match after_empty(timeout_ms, start, elapsed) {
            Next::Again => return ERRNO_AGAIN,
            Next::TimedOut => return ERRNO_TIMEDOUT,
            Next::SleepUntil(at) => at,
        };
        crate::sched::sleep_until_unless_woken(pid, deadline, token);
        if let Some(msg) = nonos_inbox::try_dequeue_existing(inbox) {
            super::pending_reply::mark_served(pid, msg.correlation);
            return deliver(pid, &msg, buf, len, outs);
        }
        crate::sched::yield_now();
    }
}
