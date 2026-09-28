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

use core::mem;

use crate::linux::abi::errno::{
    EBADF, ECONNREFUSED, EDESTADDRREQ, EINVAL, EMSGSIZE, ENOTCONN, EPERM,
};

use super::gram_dest::Dest;
use super::name::Peer;
use super::table::Socks;
use super::types::Domain;

/// The largest UDP payload IPv4 carries.
const MAX_GRAM: usize = 65507;

impl Socks {
    /// One datagram from `id`. One nobody holds the port of is dropped, as
    /// on the wire; a connected sender learns of it as ECONNREFUSED on its
    /// next call, which is what the ICMP reply does on Linux.
    pub fn send_gram(&mut self, id: u32, dest: Dest, bytes: &[u8]) -> Result<usize, i64> {
        let s = self.get_mut(id).ok_or(EBADF)?;
        if s.error != 0 {
            return Err(mem::take(&mut s.error));
        }
        if bytes.len() > MAX_GRAM {
            return Err(EMSGSIZE);
        }
        let from = match s.domain {
            Domain::Inet => Peer::Inet(s.local.unwrap_or_default()),
            Domain::Unix => Peer::Unix(s.uname.clone()),
        };
        let (remote, peer, connected, domain) = (s.remote, s.peer, s.connected, s.domain);
        let target = match (dest, domain) {
            (Dest::Sock(t), Domain::Unix) => t,
            (Dest::Default, Domain::Unix) => match peer {
                Some(p) => p,
                None if connected => return Err(ECONNREFUSED),
                None => return Err(ENOTCONN),
            },
            (Dest::Inet(to), Domain::Inet) => match self.inet_target(id, to, remote.is_some()) {
                Some(r) => r,
                None => return Ok(bytes.len()),
            },
            (Dest::Default, Domain::Inet) => match remote {
                Some(to) => match self.inet_target(id, to, true) {
                    Some(r) => r,
                    None => return Ok(bytes.len()),
                },
                None => return Err(EDESTADDRREQ),
            },
            _ => return Err(EINVAL),
        };
        let r = self.get_mut(target).ok_or(ECONNREFUSED)?;
        // A connected Unix socket takes only from its peer, and says so.
        if r.domain == Domain::Unix && r.peer.is_some_and(|p| p != id) && r.connected {
            return Err(EPERM);
        }
        let queued: usize = r.grams.iter().map(|(_, g)| g.len()).sum();
        let wanted = match (&from, r.remote) {
            (Peer::Inet(a), Some(x)) => *a == x,
            _ => true,
        };
        if wanted && queued + bytes.len() <= r.opts.rcvbuf as usize {
            r.grams.push_back((from, bytes.to_vec()));
        }
        Ok(bytes.len())
    }
}
