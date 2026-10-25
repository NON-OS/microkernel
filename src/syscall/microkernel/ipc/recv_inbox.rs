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
use crate::syscall::microkernel::errnos::{ERRNO_AGAIN, ERRNO_NOENT, ERRNO_TIMEDOUT};

use super::recv_take::taken;
use super::wait_rule::{after_empty, Next};

static NO_INBOX: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.recv");
// The first 64 dequeues systemwide, then sampled: enough to show which inboxes
// actually drain during bring-up without narrating every message after.
static FIRST_DRAIN: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.drain");

pub(super) fn recv_from_inbox(
    pid: u32,
    inbox_name: &str,
    buf: u64,
    len: usize,
    timeout_ms: u64,
) -> i64 {
    if !nonos_inbox::exists(&inbox_name) {
        // A server receiving on an inbox that does not exist is unreachable
        // forever and every caller reads as starvation. Loud, not trace-gated.
        NO_INBOX.reject(inbox_name, "missing inbox", pid);
        return ERRNO_NOENT;
    }
    let start = crate::time::timestamp_millis();
    loop {
        // The wake token is read before the queue check. A delivery bumps the
        // token and then enqueues under the same route, so a wake landing
        // after this read blocks the sleep below, and one landing before it
        // left its message where the very next check finds it. Without the
        // token there is a gap between the empty check and the transition to
        // Sleeping, and a message arriving in that gap has spent its only
        // wake on a still-Running process: the receiver then sleeps on a
        // queue that has data, forever if the timeout is infinite.
        let token = crate::sched::wake_token(pid);
        if let Some(msg) = nonos_inbox::try_dequeue_existing(&inbox_name) {
            // Logged as sender -> receiver so a flooded inbox names its
            // flooder, not just itself.
            FIRST_DRAIN.note(&msg.from, pid as u64);
            return taken(pid, &msg, buf, len);
        }
        let elapsed = crate::time::timestamp_millis().saturating_sub(start);
        let deadline = match after_empty(timeout_ms, start, elapsed) {
            Next::Again => return ERRNO_AGAIN,
            Next::TimedOut => return ERRNO_TIMEDOUT,
            Next::SleepUntil(at) => at,
        };
        crate::sched::sleep_until_unless_woken(pid, deadline, token);
        if let Some(msg) = nonos_inbox::try_dequeue_existing(&inbox_name) {
            return taken(pid, &msg, buf, len);
        }
        crate::sched::yield_now();
    }
}
