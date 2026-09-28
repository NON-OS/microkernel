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

//! Joining two stream ends: a connect on loopback or to a Unix name, which
//! Linux completes in the caller's own call, and socketpair.

use super::table::Socks;
use super::types::{Addr, Proto};

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
        match self.listener(to) {
            Some(l) => self.join(id, l),
            None => Link::Refused,
        }
    }

    /// Connect stream `id` to listener `l`: a new server end, with the
    /// listener's name and options, waits in its queue for accept.
    pub fn join(&mut self, id: u32, l: u32) -> Link {
        let Some(ls) = self.get(l) else {
            return Link::Refused;
        };
        // Linux queues one more than the backlog it was given.
        if ls.pending.len() > ls.backlog {
            return Link::Full;
        }
        let (domain, local, uname, opts) = (ls.domain, ls.local, ls.uname.clone(), ls.opts);
        let (from, from_name) = self.get(id).map_or((None, None), |c| (c.local, c.uname.clone()));
        let server = self.open(domain, Proto::Stream, None);
        if let Some(s) = self.get_mut(server) {
            s.local = local;
            s.remote = from;
            s.uname = uname.clone();
            s.upeer = from_name;
            s.peer = Some(id);
            s.connected = true;
            // An accepted socket starts with its listener's options.
            s.opts = opts;
        }
        if let Some(c) = self.get_mut(id) {
            c.remote = local;
            c.upeer = uname;
            c.peer = Some(server);
            c.connected = true;
        }
        if let Some(l) = self.get_mut(l) {
            l.pending.push_back(server);
        }
        Link::Done
    }
}
