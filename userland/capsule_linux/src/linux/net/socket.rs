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
use super::policy::refuse;
use super::sock::{self, Domain, Proto};
use super::sockaddr::{AF_INET, AF_UNIX};

const SOCK_STREAM: u64 = 1;
const SOCK_DGRAM: u64 = 2;
const SOCK_RAW: u64 = 3;
const SOCK_SEQPACKET: u64 = 5;
const TYPE_MASK: u64 = 0xF;
const IPPROTO_TCP: u64 = 6;
const IPPROTO_UDP: u64 = 17;
/// The one protocol a Unix socket takes besides 0.
const PF_UNIX: u64 = 1;

pub fn socket(guest: &mut Guest, family: u64, kind: u64, protocol: u64) -> u64 {
    let flags = kind & (SOCK_NONBLOCK | SOCK_CLOEXEC);
    if kind & !(TYPE_MASK | flags) != 0 {
        return errno::fail(errno::EINVAL);
    }
    let domain = match family {
        f if f == u64::from(AF_INET) => Domain::Inet,
        f if f == u64::from(AF_UNIX) => Domain::Unix,
        _ => return errno::fail(errno::EAFNOSUPPORT),
    };
    let (proto, own) = match (kind & TYPE_MASK, domain) {
        (SOCK_STREAM, Domain::Inet) => (Proto::Stream, IPPROTO_TCP),
        (SOCK_DGRAM, Domain::Inet) => (Proto::Dgram, IPPROTO_UDP),
        (SOCK_RAW, Domain::Inet) => {
            return refuse("SOCK_RAW: raw sockets reach below any confinement", errno::EPERM)
        }
        (SOCK_STREAM, Domain::Unix) => (Proto::Stream, PF_UNIX),
        /* Linux gives a raw Unix socket datagram semantics. */
        (SOCK_DGRAM | SOCK_RAW, Domain::Unix) => (Proto::Dgram, PF_UNIX),
        (SOCK_SEQPACKET, Domain::Unix) => {
            return refuse(
                "SOCK_SEQPACKET: a Unix stream here does not keep message boundaries",
                errno::ESOCKTNOSUPPORT,
            )
        }
        _ => return errno::fail(errno::ESOCKTNOSUPPORT),
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
