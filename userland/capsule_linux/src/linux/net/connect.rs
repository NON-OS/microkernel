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

//! `connect`. On 127.0.0.0/8 the family links the two ends in the caller's
//! own call, as Linux's loopback does, and a non-blocking socket answers
//! EINPROGRESS all the same; anywhere else a stream goes over the mixnet.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::fd::{nonblock, sock_of};
use super::sock::{self, Domain, Proto};
use super::sockaddr::{self, is_loopback, AF_INET};

pub fn connect(guest: &mut Guest, fd: u64, at: u64, len: u64) -> u64 {
    // This capsule is the nameserver, so its socket has nothing to reach.
    if guest.fds.get(fd as usize).is_some_and(|f| f.kind == Kind::Resolver) {
        return errno::ok(0);
    }
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let (family, to) = match sockaddr::read(guest, at, len) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let Some((proto, domain)) = sock::with(|t| t.get(id).map(|s| (s.proto, s.domain))) else {
        return errno::fail(errno::EBADF);
    };
    match (proto, domain) {
        (_, Domain::Unix) => super::unix_calls::connect(guest, fd, id, proto, at, len),
        (Proto::Dgram, _) => super::connect_dgram::connect(guest, fd, id, family, to),
        _ if family != AF_INET => errno::fail(errno::EAFNOSUPPORT),
        _ if is_loopback(to.ip) => super::connect_lo::loopback(id, to, nonblock(guest, fd)),
        _ => super::connect_out::connect(guest, id, to),
    }
}
