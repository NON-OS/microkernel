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

//! A RouteStream shaped as the real one, its far end played by a test.

use crate::refusal::Proxy;

/// The far end, as a test plays it.
pub trait Peer {
    fn write_all(&mut self, data: &[u8]) -> Result<(), &'static str>;
    /// What has arrived, waiting up to the wait for the first of it.
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str>;
}

/// A stream through a proxy, which names its network.
pub struct Tunnel {
    proxy: Proxy,
    peer: Box<dyn Peer>,
}

impl Tunnel {
    pub fn new(proxy: Proxy, peer: Box<dyn Peer>) -> Self {
        Self { proxy, peer }
    }

    pub fn proxy(&self) -> Proxy {
        self.proxy
    }
}

pub enum RouteStream {
    Direct(Box<dyn Peer>),
    Proxied(Tunnel),
}

impl RouteStream {
    pub fn write_all(&mut self, data: &[u8]) -> Result<(), &'static str> {
        match self {
            RouteStream::Direct(p) => p.write_all(data),
            RouteStream::Proxied(t) => t.peer.write_all(data),
        }
    }

    pub fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        match self {
            RouteStream::Direct(p) => p.read_wait(into, wait_ms),
            RouteStream::Proxied(t) => t.peer.read_wait(into, wait_ms),
        }
    }
}
