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

//! What accept takes off a listener: its oldest queued connection, held from
//! then by the process that took it, and the name it came from.

use crate::linux::abi::errno::{EAGAIN, EBADF, ECONNABORTED, EINVAL, EOPNOTSUPP};

use super::name::Peer;
use super::table::Socks;
use super::types::{Domain, Proto};

impl Socks {
    /// The next connection listener `id` queued, now held by `pid`.
    pub fn accept(&mut self, id: u32, pid: u32) -> Result<(u32, Peer), i64> {
        let s = self.get_mut(id).ok_or(EBADF)?;
        if s.proto == Proto::Dgram {
            return Err(EOPNOTSUPP);
        }
        if !s.listening {
            return Err(EINVAL);
        }
        let child = s.pending.pop_front().ok_or(EAGAIN)?;
        self.make_room(id);
        let c = self.get_mut(child).ok_or(ECONNABORTED)?;
        c.holders.push(pid);
        let from = match c.domain {
            Domain::Inet => Peer::Inet(c.remote.unwrap_or_default()),
            Domain::Unix => Peer::Unix(c.upeer.clone()),
        };
        Ok((child, from))
    }
}
