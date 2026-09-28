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
use super::sock::{self, Addr, Domain};

pub fn getsockname(guest: &mut Guest, fd: u64, at: u64, lenp: u64) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let Some((domain, local)) = sock::with(|t| t.get(id).map(|s| (s.domain, s.local))) else {
        return errno::fail(errno::EBADF);
    };
    // A socket not yet bound is 0.0.0.0, port 0.
    super::sockaddr_out::write(guest, at, lenp, domain, local.unwrap_or_default())
}

pub fn getpeername(guest: &mut Guest, fd: u64, at: u64, lenp: u64) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    // A reset connection is closed, and has no peer; one whose peer only
    // shut down, or left cleanly, still does.
    let peer: Option<(Domain, Addr)> = sock::with(|t| {
        let s = t.get(id)?;
        let live = s.connected && !s.broken && s.error == 0;
        live.then(|| (s.domain, s.remote.unwrap_or_default()))
    });
    match peer {
        Some((domain, addr)) => super::sockaddr_out::write(guest, at, lenp, domain, addr),
        None => errno::fail(errno::ENOTCONN),
    }
}
