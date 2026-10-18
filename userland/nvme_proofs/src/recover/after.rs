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

//! A server a recovery has run on, what its admin queue was sent, and
//! senders streaming reads on it.

use super::inbox::{Inbox, Reply};
use super::server::Server;
use crate::fake::register;
use crate::nvm::sq_tail_doorbell;

/// A read that failed: ms after the start, its status, whether it was sent.
pub(super) type Failed = (u64, Result<(), i32>, bool);

/// The first pid that reads once and is gone; those below it stream.
pub(super) const ONE_OFF: u32 = 50;

/// A read whose command 1 stalls with the admin queue quiet, so recovery
/// waits out an unanswered Abort before the reset; with `slow` clock reads
/// for RDY to follow CC.EN each way, and CAP.TO at its most (127.5 s), a
/// longer one. The server, and that read's reply.
pub(super) fn after_a_recovery(slow: u64) -> (Server, Reply) {
    let mut s = Server::new(crate::fake::up());
    if slow != 0 {
        s.device.info.cap |= 0xff << 24;
        s.fake.borrow_mut().slow = slow;
    }
    (s.fake.borrow_mut().stall, s.fake.borrow_mut().mute) = (vec![1], true);
    let first = s.read(None);
    assert_eq!(first.status, Err(crate::protocol::E_TIMEDOUT));
    (s, first)
}

/// Serve the inbox `q`, each streaming sender's next read queued as its
/// reply goes, until a reply `ms` after `from`; the first that failed.
pub(super) fn streamed(s: &mut Server, q: &mut Inbox, from: u64, ms: u64) -> Option<Failed> {
    loop {
        let (sender, tail) =
            (q.front().expect("queued").1, register(s.device.regs, sq_tail_doorbell(1, 0)));
        let r = s.serve_next(q).expect("queued");
        if r.status.is_err() || r.at - from > ms {
            return r.status.is_err().then_some((r.at - from, r.status, r.sent != tail));
        }
        if sender < ONE_OFF {
            q.push_back((r.at, sender));
        }
    }
}

impl Server {
    /// Admin opcodes fetched since the first `from`.
    pub fn ops_since(&self, from: usize) -> Vec<u32> {
        let seen = &self.fake.borrow().admin_seen;
        seen.get(from..).unwrap_or_default().iter().map(|c| c.cdw0 & 0xff).collect()
    }
}
