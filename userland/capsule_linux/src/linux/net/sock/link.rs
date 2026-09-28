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

//! Joining two stream ends: a loopback connect, which Linux completes in the
//! caller's own call, and socketpair.

use super::table::Socks;
use super::types::{Addr, Domain, Proto};

pub enum Link {
    /// Connected; the listener has one more connection to accept.
    Done,
    /// Nothing listens there: Linux's loopback answers with a reset.
    Refused,
    /// The listener's queue is full.
    Full,
}

impl Socks {
    /// Connect stream `id`, already bound, to the listener at `to`.
    pub fn link(&mut self, id: u32, to: Addr) -> Link {
        let Some(l) = self.listener(to) else {
            return Link::Refused;
        };
        let Some((backlog, queued, opts)) =
            self.get(l).map(|s| (s.backlog, s.pending.len(), s.opts))
        else {
            return Link::Refused;
        };
        // Linux queues one more than the backlog it was given.
        if queued > backlog {
            return Link::Full;
        }
        let from = self.get(id).and_then(|s| s.local);
        let server = self.open(Domain::Inet, Proto::Stream, None);
        if let Some(s) = self.get_mut(server) {
            s.local = Some(to);
            s.remote = from;
            s.peer = Some(id);
            s.connected = true;
            // An accepted socket starts with its listener's options.
            s.opts = opts;
        }
        if let Some(c) = self.get_mut(id) {
            c.remote = Some(to);
            c.peer = Some(server);
            c.connected = true;
        }
        if let Some(l) = self.get_mut(l) {
            l.pending.push_back(server);
        }
        Link::Done
    }

    /// Two connected sockets, both held by `pid`.
    pub fn pair(&mut self, domain: Domain, proto: Proto, pid: u32) -> (u32, u32) {
        let a = self.open(domain, proto, Some(pid));
        let b = self.open(domain, proto, Some(pid));
        for (me, other) in [(a, b), (b, a)] {
            if let Some(s) = self.get_mut(me) {
                s.peer = Some(other);
                s.connected = true;
            }
        }
        (a, b)
    }
}
