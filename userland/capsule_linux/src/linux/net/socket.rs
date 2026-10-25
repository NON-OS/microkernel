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

//! `socket` for AF_INET and AF_UNIX. The socket is the family's until it
//! connects outside 127.0.0.0/8 (then net.sockets or net.anon holds the
//! stream, guest_route.rs) or to the display's path (then it is the display
//! connection).

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::fd::{install, SOCK_CLOEXEC, SOCK_NONBLOCK};
use super::sock::{self, Domain};
use super::sockaddr::{AF_INET, AF_UNIX};
use super::socket_kind::TYPE_MASK;

pub fn socket(guest: &mut Guest, family: u64, kind: u64, protocol: u64) -> u64 {
    let flags = kind & (SOCK_NONBLOCK | SOCK_CLOEXEC);
    if kind & !(TYPE_MASK | flags) != 0 {
        return errno::fail(errno::EINVAL);
    }
    let domain = match family {
        /* A learn run opens one, so its connect is refused naming where it went. */
        f if f == u64::from(AF_INET) && !crate::linux::app::inet(guest) && !learning() => {
            return crate::linux::app::refuse(
                "an internet socket",
                "NET_ANON or NET_LISTEN_LO",
                errno::EACCES,
            )
        }
        f if f == u64::from(AF_INET) => Domain::Inet,
        f if f == u64::from(AF_UNIX) => Domain::Unix,
        _ => return errno::fail(errno::EAFNOSUPPORT),
    };
    let (proto, own) = match super::socket_kind::kind(kind, domain) {
        Ok(k) => k,
        Err(e) => return e,
    };
    /*
     * Anything but the type's own protocol, MPTCP included, is one this
     * stack does not have; Go falls back to TCP on this answer.
     */
    if protocol != 0 && protocol != own {
        return errno::fail(errno::EPROTONOSUPPORT);
    }
    let id = sock::with(|t| t.open(domain, proto, Some(guest.pid)));
    install(guest, id, flags)
}

fn learning() -> bool {
    crate::linux::boxed::learning()
}
