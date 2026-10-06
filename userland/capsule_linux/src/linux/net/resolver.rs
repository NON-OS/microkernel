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

//! A datagram socket that talks to a nameserver. This capsule answers name
//! queries itself (`dns`), so a query never leaves it: a socket that sends
//! to port 53 outside the family, or to a loopback port 53 no family socket
//! holds, becomes the resolver descriptor it always was before a guest
//! could bind a datagram socket of its own.

use crate::linux::guest::{Fd, Guest};

use super::dgram_addr::{encode, fill};
use super::dns;

use super::sock::{self, Addr, Proto};
use super::sockaddr::is_loopback;

const NAMESERVER_PORT: u16 = 53;

pub fn is_nameserver(to: Addr) -> bool {
    to.port == NAMESERVER_PORT
        && (!is_loopback(to.ip) || sock::with(|t| t.bound(Proto::Dgram, to).is_none()))
}

/// Let go of the socket `fd` names and make `fd` the resolver, keeping its
/// descriptor flags.
pub fn become_resolver(guest: &mut Guest, fd: u64) {
    super::close::close(guest, fd);
    if let Some(f) = guest.fds.get_mut(fd as usize) {
        let (cloexec, nonblock) = (f.cloexec, f.nonblock);
        *f = Fd::resolver();
        f.cloexec = cloexec;
        f.nonblock = nonblock;
    }
}

/// A query written to the resolver. A program with no `resolv.conf` asks
/// the loopback address, and one with a configured nameserver asks that.
pub fn query(guest: &mut Guest, fd: u64, buf: u64, len: u64, to: Option<Addr>) -> u64 {
    let peer = to.map_or((NAMESERVER_PORT, [127, 0, 0, 1]), |a| (a.port, a.ip));
    dns::query(guest, fd, buf, len, encode(peer))
}

/// An answer read from the resolver, with the nameserver it came from.
pub fn answer(guest: &mut Guest, fd: u64, buf: u64, len: u64, at: u64, alen: u64) -> u64 {
    let (got, from) = dns::answer_out(guest, fd, buf, len);
    match from {
        Some(peer) if at != 0 => fill(guest, at, alen, peer, got),
        _ => got,
    }
}
