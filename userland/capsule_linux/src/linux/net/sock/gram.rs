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

//! A datagram into the family: to whichever socket holds the port or the
//! Unix name it is sent to, or to a connected socket's peer.

use crate::linux::abi::errno::{ECONNREFUSED, EPERM};

use super::gram_dest::Dest;
use super::name::{Gram, Peer};
use super::table::Socks;
use super::types::Domain;

impl Socks {
    /// One datagram from `id`. One nobody holds the port of is dropped, as
    /// on the wire.
    pub fn send_gram(&mut self, id: u32, dest: Dest, bytes: &[u8]) -> Result<usize, i64> {
        let target = self.gram_target(id, dest, bytes.len())?;
        let (Some(t), Some(from)) = (target, self.sender(id)) else {
            return Ok(bytes.len());
        };
        let r = self.get_mut(t).ok_or(ECONNREFUSED)?;
        /* A connected Unix socket takes only from its peer, and says so. */
        if r.domain == Domain::Unix && r.connected && r.peer.is_some_and(|p| p != id) {
            return Err(EPERM);
        }
        let queued: usize = r.grams.iter().map(|g| g.bytes.len()).sum();
        let wanted = match (&from, r.remote) {
            (Peer::Inet(a), Some(x)) => *a == x,
            _ => true,
        };
        if wanted && queued + bytes.len() <= r.opts.rcvbuf as usize {
            r.grams.push_back(Gram { from, bytes: bytes.to_vec() });
        }
        Ok(bytes.len())
    }
}
