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

//! The value getsockopt reads for each option a socket has.

use alloc::vec::Vec;

use crate::linux::abi::errno;

use super::ids::*;
use crate::linux::net::sock::{Domain, Proto, Sock};

pub fn value(s: &mut Sock, level: u64, name: u64) -> Result<Vec<u8>, u64> {
    let o = s.opts;
    let int = |v: u32| Ok(v.to_le_bytes().to_vec());
    let pair = |a: u64, b: u64| Ok([a.to_le_bytes(), b.to_le_bytes()].concat());
    let stream = s.proto == Proto::Stream;
    match (level, name) {
        (SOL_SOCKET, SO_TYPE) => int(if stream { 1 } else { 2 }),
        (SOL_SOCKET, SO_DOMAIN) => int(if s.domain == Domain::Unix { 1 } else { 2 }),
        (SOL_SOCKET, SO_PROTOCOL) => match (s.domain, stream) {
            (Domain::Unix, _) => int(0),
            (_, true) => int(6),
            (_, false) => int(17),
        },
        (SOL_SOCKET, SO_ACCEPTCONN) => int(u32::from(s.listening)),
        (SOL_SOCKET, SO_ERROR) => int(core::mem::take(&mut s.error) as u32),
        (SOL_SOCKET, SO_REUSEADDR) => int(u32::from(o.reuseaddr)),
        (SOL_SOCKET, SO_REUSEPORT) => int(u32::from(o.reuseport)),
        (SOL_SOCKET, SO_KEEPALIVE) => int(u32::from(o.keepalive)),
        (SOL_SOCKET, SO_BROADCAST) => int(u32::from(o.broadcast)),
        (SOL_SOCKET, SO_RCVBUF) => int(o.rcvbuf),
        (SOL_SOCKET, SO_SNDBUF) => int(o.sndbuf),
        (SOL_SOCKET, SO_LINGER) => {
            Ok([o.linger.0.to_le_bytes(), o.linger.1.to_le_bytes()].concat())
        }
        (SOL_SOCKET, SO_RCVTIMEO) => pair(o.rcvtimeo.0, o.rcvtimeo.1),
        (SOL_SOCKET, SO_SNDTIMEO) => pair(o.sndtimeo.0, o.sndtimeo.1),
        (l, _) if s.domain == Domain::Unix && l != SOL_SOCKET => {
            Err(errno::fail(errno::EOPNOTSUPP))
        }
        (IPPROTO_TCP, _) if !stream && super::more::known(level, name) => {
            Err(errno::fail(errno::EOPNOTSUPP))
        }
        (l, n) if super::more::known(l, n) => int(super::more::get(&o.more, l, n)),
        /* A datagram socket has no TCP options, and Linux says so this way. */
        (IPPROTO_TCP, _) if !stream => Err(errno::fail(errno::EOPNOTSUPP)),
        (IPPROTO_TCP, TCP_NODELAY) => int(u32::from(o.nodelay)),
        (IPPROTO_TCP, TCP_KEEPIDLE) => int(o.keepidle),
        (IPPROTO_TCP, TCP_KEEPINTVL) => int(o.keepintvl),
        (IPPROTO_TCP, TCP_KEEPCNT) => int(o.keepcnt),
        (IPPROTO_IPV6, _) => Err(errno::fail(errno::EOPNOTSUPP)),
        _ => Err(super::get::unknown("getsockopt", level, name)),
    }
}
