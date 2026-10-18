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

//! The server loop (server/runner.rs) over an inbox the test fills: each
//! request is taken by the driver's own server::take, its receive played
//! here, and served as a one-block read for the pid that sent it (0: the
//! kernel), whose reply is noted as it goes. The clock moves 100 ms a read,
//! so a receive waits its timeout in reads, a 1 ms one a single read.

use std::collections::VecDeque;

use super::server::Server;
use crate::server::take::{take, ERRNO_TIMEDOUT};

/// A reply: its status, the milliseconds from the request's start to it, the
/// clock when it went, and the admin and I/O commands sent by then (queue
/// 1's tail, which a reset puts back to 0).
pub(super) struct Reply {
    pub status: Result<(), i32>,
    pub ms: u64,
    pub at: u64,
    pub admin: usize,
    pub sent: u32,
}

/// Requests: the clock time each is queued at, and its sender's pid.
pub(super) type Inbox = VecDeque<(u64, u32)>;

/// The kernel's requests, queued at the times `at`.
pub(super) fn kernel(at: impl IntoIterator<Item = u64>) -> Inbox {
    at.into_iter().map(|at| (at, 0)).collect()
}

impl Server {
    /// Take the next request and serve it; None once nothing more is ever
    /// queued, where the server would wait for good.
    pub fn serve_next(&mut self, inbox: &mut Inbox) -> Option<Reply> {
        let fake = self.fake.clone();
        let now = move || fake.borrow().ticks * 100;
        let mut pid = None;
        let got = take(self.io.as_mut(), |timeout| {
            let due = |inbox: &Inbox| inbox.front().is_some_and(|&(at, _)| at <= now());
            let until = now() + timeout;
            while !due(inbox) && (now() < until || (timeout == 0 && !inbox.is_empty())) {
                nonos_libc::mk_uptime_ms();
            }
            if due(inbox) {
                pid = inbox.pop_front().map(|(_, p)| p);
                1
            } else if timeout > 0 {
                ERRNO_TIMEDOUT
            } else {
                -1
            }
        });
        pid.filter(|_| got > 0).map(|pid| self.read_as(None, pid))
    }
}
