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

//! `socket` for AF_INET. The socket is the family's until it connects
//! outside 127.0.0.0/8; only then is net.sockets asked for one.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::fd::{install, SOCK_CLOEXEC, SOCK_NONBLOCK};
use super::policy::refuse;
use super::sock::{self, Domain, Proto};
use super::sockaddr::AF_INET;

const SOCK_STREAM: u64 = 1;
const SOCK_DGRAM: u64 = 2;
const SOCK_RAW: u64 = 3;
const TYPE_MASK: u64 = 0xF;
const IPPROTO_TCP: u64 = 6;
const IPPROTO_UDP: u64 = 17;

pub fn socket(guest: &mut Guest, family: u64, kind: u64, protocol: u64) -> u64 {
    let flags = kind & (SOCK_NONBLOCK | SOCK_CLOEXEC);
    if kind & !(TYPE_MASK | flags) != 0 {
        return errno::fail(errno::EINVAL);
    }
    if family != u64::from(AF_INET) {
        return errno::fail(errno::EAFNOSUPPORT);
    }
    let (proto, own) = match kind & TYPE_MASK {
        SOCK_STREAM => (Proto::Stream, IPPROTO_TCP),
        SOCK_DGRAM => (Proto::Dgram, IPPROTO_UDP),
        SOCK_RAW => {
            return refuse("SOCK_RAW: raw sockets reach below any confinement", errno::EPERM)
        }
        _ => return errno::fail(errno::ESOCKTNOSUPPORT),
    };
    // Anything but the type's own protocol, MPTCP included, is one this
    // stack does not have; Go falls back to TCP on this answer.
    if protocol != 0 && protocol != own {
        return errno::fail(errno::EPROTONOSUPPORT);
    }
    let id = sock::with(|t| t.open(Domain::Inet, proto, Some(guest.pid)));
    install(guest, id, flags)
}
