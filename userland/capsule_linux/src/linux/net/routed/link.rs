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

//! An open stream through net.socks5: what the guest sent and the proxy
//! has not taken, what came back and the guest has not read, and how it
//! ended. It is carried in slices (pump.rs).

use alloc::vec::Vec;

use nonos_route_link::RouteStream;

use super::super::sock::End;

/// The most a guest's bytes queue here before a send waits.
pub const TX_MAX: usize = 64 << 10;

pub struct Link {
    pub stream: u32,
    pub(super) route: RouteStream,
    pub tx: Vec<u8>,
    /// How many of `tx` the frame still unanswered carries, 0 for none.
    pub(super) offered: usize,
    /// What came back and the guest has not read; filled only when empty.
    pub rx: Vec<u8>,
    pub end: Option<End>,
    /// A write failed: nothing more is sent, and what came is still read.
    pub broken: bool,
}

impl Link {
    pub fn new(stream: u32, route: RouteStream) -> Link {
        let (tx, rx) = (Vec::new(), Vec::new());
        Link { stream, route, tx, offered: 0, rx, end: None, broken: false }
    }
}
