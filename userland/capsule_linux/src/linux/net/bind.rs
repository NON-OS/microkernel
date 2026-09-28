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

//! `bind` and `listen`, on 127.0.0.0/8 only (`policy`).

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::fd::sock_of;
use super::policy::not_loopback;
use super::sock::{self, Domain};
use super::sockaddr::{self, is_loopback, AF_INET};

pub fn bind(guest: &mut Guest, fd: u64, at: u64, len: u64) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    if sock::with(|t| t.get(id).is_some_and(|s| s.domain == Domain::Unix)) {
        return super::unix_bind::bind(guest, id, at, len);
    }
    let (family, mut want) = match sockaddr::read(guest, at, len) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if family != AF_INET {
        return errno::fail(errno::EAFNOSUPPORT);
    }
    if !is_loopback(want.ip) {
        return not_loopback("bind", want);
    }
    sock::with(|t| {
        let Some(s) = t.get(id) else {
            return errno::fail(errno::EBADF);
        };
        if s.domain != Domain::Inet || s.local.is_some() || s.svc.is_some() {
            return errno::fail(errno::EINVAL);
        }
        if want.port == 0 {
            match t.ephemeral(s.proto, want.ip) {
                Some(port) => want.port = port,
                None => return errno::fail(errno::EADDRINUSE),
            }
        } else if t.in_use(id, want) {
            return errno::fail(errno::EADDRINUSE);
        }
        if let Some(s) = t.get_mut(id) {
            s.local = Some(want);
        }
        errno::ok(0)
    })
}
