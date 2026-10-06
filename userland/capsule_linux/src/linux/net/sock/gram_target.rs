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

//! Which socket a datagram goes to, or why it goes nowhere.

use core::mem;

use crate::linux::abi::errno::{EBADF, ECONNREFUSED, EDESTADDRREQ, EINVAL, EMSGSIZE, ENOTCONN};

use super::gram_dest::Dest;
use super::name::Peer;
use super::table::Socks;
use super::types::Domain;

/// The largest UDP payload IPv4 carries.
const MAX_GRAM: usize = 65507;

impl Socks {
    /// The socket a datagram from `id` goes to; None drops it unseen.
    pub(super) fn gram_target(
        &mut self,
        id: u32,
        dest: Dest,
        len: usize,
    ) -> Result<Option<u32>, i64> {
        let s = self.get_mut(id).ok_or(EBADF)?;
        if s.error != 0 {
            return Err(mem::take(&mut s.error));
        }
        if len > MAX_GRAM {
            return Err(EMSGSIZE);
        }
        let (remote, peer, connected, domain) = (s.remote, s.peer, s.connected, s.domain);
        match (dest, domain) {
            (Dest::Sock(t), Domain::Unix) => Ok(Some(t)),
            (Dest::Default, Domain::Unix) => match peer {
                Some(p) => Ok(Some(p)),
                None if connected => Err(ECONNREFUSED),
                None => Err(ENOTCONN),
            },
            (Dest::Inet(to), Domain::Inet) => Ok(self.inet_target(id, to, remote.is_some())),
            (Dest::Default, Domain::Inet) => match remote {
                Some(to) => Ok(self.inet_target(id, to, true)),
                None => Err(EDESTADDRREQ),
            },
            _ => Err(EINVAL),
        }
    }

    /// How a datagram from `id` names its sender.
    pub(super) fn sender(&self, id: u32) -> Option<Peer> {
        self.get(id).map(|s| match s.domain {
            Domain::Inet => Peer::Inet(s.local.unwrap_or_default()),
            Domain::Unix => Peer::Unix(s.uname.clone()),
        })
    }
}
