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

//! `accept` and `accept4`: the oldest connection a listener has queued, as
//! a new descriptor held by the caller alone.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::close::discard;
use super::fd::{install, sock_of, SOCK_CLOEXEC, SOCK_NONBLOCK};
use super::sock::{self, Domain, Peer, Proto};

pub fn accept4(guest: &mut Guest, fd: u64, at: u64, lenp: u64, flags: u64) -> u64 {
    if flags & !(SOCK_NONBLOCK | SOCK_CLOEXEC) != 0 {
        return errno::fail(errno::EINVAL);
    }
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let pid = guest.pid;
    let taken = sock::with(|t| {
        let s = t.get_mut(id).ok_or(errno::EBADF)?;
        if s.proto == Proto::Dgram {
            return Err(errno::EOPNOTSUPP);
        }
        if !s.listening {
            return Err(errno::EINVAL);
        }
        let child = s.pending.pop_front().ok_or(errno::EAGAIN)?;
        let c = t.get_mut(child).ok_or(errno::ECONNABORTED)?;
        c.holders.push(pid);
        let from = match c.domain {
            Domain::Inet => Peer::Inet(c.remote.unwrap_or_default()),
            Domain::Unix => Peer::Unix(c.upeer.clone()),
        };
        Ok((child, from))
    });
    let (child, from) = match taken {
        Ok(v) => v,
        Err(e) => return errno::fail(e),
    };
    let n = install(guest, child, flags);
    let Some(slot) = errno::slot(n) else {
        return n;
    };
    let wrote = super::sockaddr_out::write(guest, at, lenp, &from);
    if errno::slot(wrote).is_none() {
        discard(guest, slot as u64);
        return wrote;
    }
    n
}
