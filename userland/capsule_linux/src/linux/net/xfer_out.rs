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

//! Bytes out of a socket: to a stream's peer, a datagram to the port or the
//! Unix name it is sent to, or a stream outside the family through
//! net.sockets.

use alloc::vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::flags::MSG_OOB;
use super::iov::{self, Iov};
use super::peer_addr::To;
use super::sock::{self, Proto};

/// Send the message in `iov` from socket `id`, `skip` bytes of it having
/// gone already: the count sent now, or an errno.
pub fn send(guest: &Guest, id: u32, iov: &Iov, skip: usize, flags: u64, to: Option<To>) -> u64 {
    if flags & MSG_OOB != 0 {
        return errno::fail(errno::EOPNOTSUPP);
    }
    let Some((proto, domain, svc)) = sock::with(|t| t.get(id).map(|s| (s.proto, s.domain, s.svc)))
    else {
        return errno::fail(errno::EBADF);
    };
    let cap = super::cap::cap(svc.is_some(), proto == Proto::Stream);
    let bytes = match iov::gather(guest, iov, skip, cap) {
        Ok(b) => b,
        Err(e) => return e,
    };
    if let Some(h) = svc {
        return super::stream::send_bytes(h, &bytes);
    }
    let dest = match super::dest::dest(guest, proto, domain, to) {
        Ok(d) => d,
        Err(e) => return e,
    };
    match sock::with(|t| t.deliver(id, dest, &bytes)) {
        Ok(n) => errno::ok(n as u64),
        Err(e) => errno::fail(e),
    }
}

/// `write` on a socket: a send with no flags and no address.
pub fn write(guest: &Guest, id: u32, buf: u64, len: u64) -> u64 {
    send(guest, id, &vec![(buf, len)], 0, 0, None)
}
