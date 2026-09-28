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

//! A datagram into the family: to whichever socket holds the port it is
//! sent to, or a socketpair end's peer.

use core::mem;

use crate::linux::abi::errno::{EBADF, ECONNREFUSED, EDESTADDRREQ, EMSGSIZE, ENOTCONN};

use super::table::Socks;
use super::types::{Addr, Domain, Proto};

/// The largest UDP payload IPv4 carries.
const MAX_GRAM: usize = 65507;

impl Socks {
    /// One datagram from `id` to `to`, or to the address it connected to. A
    /// datagram nobody is bound to receive is dropped, as on the wire; a
    /// connected sender learns of it as ECONNREFUSED on its next call, which
    /// is what the ICMP reply does on Linux.
    pub fn send_gram(&mut self, id: u32, to: Option<Addr>, bytes: &[u8]) -> Result<usize, i64> {
        let s = self.get_mut(id).ok_or(EBADF)?;
        if s.error != 0 {
            return Err(mem::take(&mut s.error));
        }
        if bytes.len() > MAX_GRAM {
            return Err(EMSGSIZE);
        }
        let (from, connected) = (s.local.unwrap_or_default(), s.remote.is_some());
        let target = match s.domain {
            Domain::Unix => s.peer.ok_or(ECONNREFUSED)?,
            Domain::Inet => {
                let dest =
                    to.or(s.remote).ok_or(if connected { ENOTCONN } else { EDESTADDRREQ })?;
                match self.bound(Proto::Dgram, dest) {
                    Some(r) => r,
                    None => {
                        if let Some(s) = self.get_mut(id).filter(|_| connected) {
                            s.error = ECONNREFUSED;
                        }
                        return Ok(bytes.len());
                    }
                }
            }
        };
        let r = self.get_mut(target).ok_or(ECONNREFUSED)?;
        let queued: usize = r.grams.iter().map(|(_, g)| g.len()).sum();
        let wanted = r.remote.is_none_or(|x| x == from);
        if wanted && queued + bytes.len() <= r.opts.rcvbuf as usize {
            r.grams.push_back((from, bytes.to_vec()));
        }
        Ok(bytes.len())
    }
}
