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


//! The link to the guard, as two queues.
//!
//! The capsule's real link is TLS over net.tcp. Here every cell it sends is
//! kept for the simulated network to read, and every cell the network
//! answers with waits in `inbound` for the pump. The two files the onion
//! certificate check reads from the link module are the capsule's own.

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::cell::Frame;
use crate::path::Relay;

#[path = "../../capsule_net_anon/src/link/constants.rs"]
pub(crate) mod constants;
#[path = "../../capsule_net_anon/src/link/ed_cert/mod.rs"]
pub(crate) mod ed_cert;

pub struct Link {
    /// Every cell sent, as encoded for the wire, oldest first.
    pub sent: Vec<Vec<u8>>,
    /// Frames waiting for the pump.
    pub inbound: VecDeque<Frame>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LinkError {
    Connect,
    Tls,
    Protocol,
    Version,
    Identity,
    Closed,
}

impl Link {
    pub fn send(&mut self, bytes: &[u8]) -> Result<(), LinkError> {
        self.sent.push(bytes.to_vec());
        Ok(())
    }

    pub fn recv(&mut self, _wait_ms: i64) -> Result<Option<Frame>, LinkError> {
        Ok(self.inbound.pop_front())
    }

    pub fn hold(&mut self, frame: Frame) {
        self.inbound.push_front(frame);
    }
}

/// Every dial succeeds: the guard the manager drew is the one the simulated
/// network answers as.
pub fn open(_tcp_port: u32, _relay: &Relay, _now: u64) -> Result<Link, LinkError> {
    Ok(Link { sent: Vec::new(), inbound: VecDeque::new() })
}
