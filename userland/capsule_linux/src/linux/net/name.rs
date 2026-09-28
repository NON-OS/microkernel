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

//! `getsockname` and `getpeername`.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::fd::sock_of;
use super::sock::{self, Domain, Peer};

pub fn getsockname(guest: &mut Guest, fd: u64, at: u64, lenp: u64) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    /* A socket not yet bound is 0.0.0.0 port 0, or an unnamed Unix socket. */
    let Some(me) = sock::with(|t| {
        t.get(id).map(|s| match s.domain {
            Domain::Inet => Peer::Inet(s.local.unwrap_or_default()),
            Domain::Unix => Peer::Unix(s.uname.clone()),
        })
    }) else {
        return errno::fail(errno::EBADF);
    };
    super::sockaddr_out::write(guest, at, lenp, &me)
}

pub fn getpeername(guest: &mut Guest, fd: u64, at: u64, lenp: u64) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    /*
     * A reset connection is closed, and has no peer; one whose peer only
     * shut down, or left cleanly, still does.
     */
    let peer: Option<Peer> = sock::with(|t| {
        let s = t.get(id)?;
        let live = s.connected && !s.broken && s.error == 0;
        live.then(|| match s.domain {
            Domain::Inet => Peer::Inet(s.remote.unwrap_or_default()),
            Domain::Unix => Peer::Unix(s.upeer.clone()),
        })
    });
    match peer {
        Some(p) => super::sockaddr_out::write(guest, at, lenp, &p),
        None => errno::fail(errno::ENOTCONN),
    }
}
