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
    pub next: u32,
    pub opened: Vec<u32>,
    pub closed: Vec<u32>,
    pub connects: Vec<(u32, [u8; 4], u16)>,
    pub waited: Vec<(u32, String)>,
    pub writable: bool,
    pub polls: u32,
    pub resolves: u32,
    pub names: Vec<(String, [u8; 4])>,
    pub sent: Vec<(u32, Vec<u8>)>,
    pub inbox: Vec<(u32, VecDeque<Vec<u8>>)>,
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
        let Some((_, q)) = self.inbox.iter_mut().find(|(h, _)| *h == handle) else {
            return Recv::Empty;
        };
        let Some(mut chunk) = q.pop_front() else { return Recv::Empty };
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
