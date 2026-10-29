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


//! One connection: what the client has said, and what it has not read.
//!
//! Both directions are this capsule's memory, so both are bounded as a
//! socket's buffers are. A client that sends requests and never reads what
//! they answer (a sync answers 24 bytes for 12, a get_registry 180) would
//! otherwise grow the events waiting for it without end. Past MOST_UNREAD
//! the server stops taking requests, which wait in `to_server`; past
//! MOST_UNSENT a write takes nothing and is EAGAIN, as a full socket answers,
//! until the client reads and the server goes on.

use alloc::vec::Vec;

/// The most events the server lets wait for a client before it stops
/// serving that client's requests.
pub const MOST_UNREAD: usize = 1 << 20;
/// The most request bytes a client may have waiting to be served.
pub const MOST_UNSENT: usize = 1 << 20;

pub struct Conn {
    /// Bytes the guest wrote and the server has not consumed.
    pub to_server: Vec<u8>,
    /// Bytes the server produced and the guest has not read.
    pub to_client: Vec<u8>,
    /// Descriptors the client passed in control data, in order.
    pub fds: Vec<u32>,
    /// Descriptors owed to the client, for the next control block.
    pub give: Vec<u32>,
}

impl Conn {
    pub fn new() -> Conn {
        Conn { to_server: Vec::new(), to_client: Vec::new(), fds: Vec::new(), give: Vec::new() }
    }

    /// Take everything the server has produced, up to `want`.
    pub fn drain(&mut self, want: usize) -> Vec<u8> {
        let n = want.min(self.to_client.len());
        self.to_client.drain(..n).collect()
    }

    /// Whether the client has left so much unread that the server serves
    /// none of its requests until it reads.
    pub fn backlogged(&self) -> bool {
        self.to_client.len() >= MOST_UNREAD
    }

    /// How many more request bytes a write may hand the server now.
    pub fn room(&self) -> usize {
        MOST_UNSENT.saturating_sub(self.to_server.len())
    }
}

impl Default for Conn {
    fn default() -> Conn {
        Conn::new()
    }
}
