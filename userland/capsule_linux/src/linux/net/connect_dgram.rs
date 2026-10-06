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

//! `connect` on a datagram socket: it only names where sends go and whose
//! datagrams are kept. AF_UNSPEC forgets it again.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::policy::refuse_out;
use super::sock::{self, Addr};
use super::sockaddr::{is_loopback, AF_INET, AF_UNSPEC};

pub fn connect(guest: &mut Guest, fd: u64, id: u32, family: u16, to: Addr) -> u64 {
    if family == AF_UNSPEC {
        sock::with(|t| t.get_mut(id).map(|s| s.remote = None));
        return errno::ok(0);
    }
    if family != AF_INET {
        return errno::fail(errno::EAFNOSUPPORT);
    }
    if super::resolver::is_nameserver(to) {
        super::resolver::become_resolver(guest, fd);
        return errno::ok(0);
    }
    if !is_loopback(to.ip) {
        return refuse_out("connect", to);
    }
    sock::with(|t| {
        t.autobind(id)?;
        if let Some(s) = t.get_mut(id) {
            s.remote = Some(to);
            s.error = 0;
        }
        Ok(())
    })
    .map_or_else(errno::fail, |()| errno::ok(0))
}
