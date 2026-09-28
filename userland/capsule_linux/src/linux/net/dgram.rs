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

//! `sendto` and `recvfrom`, which differ from write and read only in
//! carrying an address.

use alloc::vec;

use crate::linux::guest::{Guest, Kind};

use super::fd::sock_of;
use super::peer_addr::To;
use super::sock::{self, Domain, Proto};
use super::sockaddr::is_loopback;

pub fn sendto(
    guest: &mut Guest,
    fd: u64,
    buf: u64,
    len: u64,
    flags: u64,
    at: u64,
    alen: u64,
) -> u64 {
    let to = match super::peer_addr::address(guest, at, alen) {
        Ok(to) => to,
        Err(e) => return e,
    };
    if !is_resolver(guest, fd) {
        let id = match sock_of(guest, fd) {
            Ok(id) => id,
            Err(e) => return e,
        };
        let inet_dgram = sock::with(|t| {
            t.get(id).is_some_and(|s| s.proto == Proto::Dgram && s.domain == Domain::Inet)
        });
        match to {
            Some(To::Inet(a)) if inet_dgram && super::resolver::is_nameserver(a) => {
                super::resolver::become_resolver(guest, fd)
            }
            Some(To::Inet(a)) if inet_dgram && !is_loopback(a.ip) => {
                return super::policy::refuse_out("sendto", a)
            }
            to => return super::xfer_out::send(guest, id, &vec![(buf, len)], 0, flags, to),
        }
    }
    let to = match to {
        Some(To::Inet(a)) => Some(a),
        _ => None,
    };
    super::resolver::query(guest, fd, buf, len, to)
}

pub fn recvfrom(
    guest: &mut Guest,
    fd: u64,
    buf: u64,
    len: u64,
    flags: u64,
    at: u64,
    alen: u64,
) -> u64 {
    if is_resolver(guest, fd) {
        return super::resolver::answer(guest, fd, buf, len, at, alen);
    }
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    match super::xfer_in::recv(guest, id, &vec![(buf, len)], 0, flags) {
        Ok(got) => super::peer_addr::finish(guest, got, flags, at, alen),
        Err(e) => e,
    }
}

fn is_resolver(guest: &Guest, fd: u64) -> bool {
    guest.fds.get(fd as usize).is_some_and(|f| f.kind == Kind::Resolver)
}
