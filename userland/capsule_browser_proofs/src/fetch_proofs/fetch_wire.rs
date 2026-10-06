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

//! A scripted socket and clock, so the fetch machine runs without a network.

use std::cell::Cell;
use std::collections::VecDeque;

use crate::browser::net::{Recv, Source};

#[derive(Default)]
pub struct FakeWire {
    pub now: Cell<i64>,
    pub mixnet: bool,
    /* With `mixnet`, the route is the Anyone network rather than Nym. */
    pub anyone: bool,
    /* When set, every way is decided by these, host by host. */
    pub routes: Option<crate::browser::net::mixnet::Routes>,
    /* The way each socket was opened for, in order. */
    pub ways: Vec<crate::browser::net::mixnet::Way>,
    pub next: u32,
    pub opened: Vec<u32>,
    pub closed: Vec<u32>,
    pub connects: Vec<(u32, [u8; 4], u16)>,
    pub waited: Vec<(u32, String)>,
    pub writable: bool,
    pub polls: u32,
    pub resolves: u32,
    pub names: Vec<(String, [u8; 4])>,
    /* When set, what every lookup answers, whatever the name. */
    pub resolver: Option<crate::browser::fetch::wire::Resolved>,
    /* When set, how every connect by name fails. */
    pub host_fail: Option<crate::browser::fetch::wire::HostFail>,
    pub sent: Vec<(u32, Vec<u8>)>,
    /* Handles whose sends a proxy has not answered yet, and the asks. */
    pub unanswered: Vec<u32>,
    pub asks: u32,
    /* The proxy refuses every call: what was sent is never answered. */
    pub refusing: bool,
    /* Handles whose far end has finished: once their inbox is read, a read
     * says Closed. */
    pub finished: Vec<u32>,
    pub inbox: Vec<(u32, VecDeque<Vec<u8>>)>,
    /* Handles whose proxy broke, and the code it broke with: a read there
     * still says Closed once the inbox is empty. */
    pub broken: Vec<(u32, &'static str)>,
    /* The streams a proxy gives this program, when it is not unbounded:
     * a connection through it opens only while fewer are open. */
    pub streams: Option<usize>,
}

impl FakeWire {
    pub fn at(ms: i64) -> FakeWire {
        FakeWire { now: Cell::new(ms), next: 10, ..FakeWire::default() }
    }
    pub fn advance(&self, ms: i64) {
        self.now.set(self.now.get() + ms);
    }
    /// Make `bytes` the next read on `handle`.
    pub fn deliver(&mut self, handle: u32, bytes: &[u8]) {
        match self.inbox.iter_mut().find(|(h, _)| *h == handle) {
            Some((_, q)) => q.push_back(bytes.to_vec()),
            None => self.inbox.push((handle, VecDeque::from([bytes.to_vec()]))),
        }
    }
}

impl Source for FakeWire {
    fn recv(&mut self, handle: u32, out: &mut [u8]) -> Recv {
        let nothing = match self.finished.contains(&handle) {
            true => Recv::Closed,
            false => Recv::Empty,
        };
        let Some((_, q)) = self.inbox.iter_mut().find(|(h, _)| *h == handle) else {
            return nothing;
        };
        let Some(mut chunk) = q.pop_front() else { return nothing };
        let n = chunk.len().min(out.len());
        out[..n].copy_from_slice(&chunk[..n]);
        if n < chunk.len() {
            q.push_front(chunk.split_off(n));
        }
        Recv::Bytes(n)
    }
    fn now_ms(&self) -> i64 {
        self.now.get()
    }
}
