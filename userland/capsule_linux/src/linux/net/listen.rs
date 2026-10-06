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

//! `listen`, on a socket bound to 127.0.0.1 (`policy`).

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::fd::sock_of;
use super::policy::not_loopback;
use super::sock::{self, Addr, Domain, Proto};

pub fn listen(guest: &mut Guest, fd: u64, backlog: u64) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    sock::with(|t| {
        let Some(s) = t.get(id) else {
            return errno::fail(errno::EBADF);
        };
        match (s.proto, s.domain, s.local) {
            (Proto::Dgram, ..) => return errno::fail(errno::EOPNOTSUPP),
            _ if s.connected || s.svc.is_some() => return errno::fail(errno::EINVAL),
            /* A Unix socket must be bound first: Linux does not name it here. */
            (_, Domain::Unix, _) if s.uname.is_none() => return errno::fail(errno::EINVAL),
            (_, Domain::Unix, _) => {}
            /* Linux would bind 0.0.0.0 here, which is not the family's own. */
            (_, _, None) => return not_loopback("listen", Addr::default()),
            /* Listeners share an address only when each set SO_REUSEPORT. */
            (_, _, Some(at))
                if !s.listening
                    && t.iter().any(|(_, o)| {
                        o.listening
                            && o.local == Some(at)
                            && !(o.opts.reuseport && s.opts.reuseport)
                    }) =>
            {
                return errno::fail(errno::EADDRINUSE)
            }
            _ => {}
        }
        if let Some(s) = t.get_mut(id) {
            s.listening = true;
            /* An int, and somaxconn's 4096 is the most Linux keeps. */
            s.backlog = (backlog as i32).clamp(0, 4096) as usize;
        }
        errno::ok(0)
    })
}
