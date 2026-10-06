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

//! One option set on one socket: kept as Linux keeps it, or refused.

use crate::linux::abi::errno;
use crate::linux::net::sock::{Domain, Proto, Sock};

use super::ids::*;
use super::time::{keep, timeo};

/// `raw` holds up to sixteen bytes of the value; `len` is what the guest
/// said it gave, at least four.
pub fn apply(s: &mut Sock, (level, name): (u64, u64), raw: &[u8], len: u64) -> u64 {
    let int = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    let word =
        |i: usize| raw.get(i..i + 8).map(|b| u64::from_le_bytes(b.try_into().unwrap_or([0; 8])));
    let (proto, domain) = (s.proto, s.domain);
    let o = &mut s.opts;
    match (level, name) {
        /* A Unix socket has only socket-level options on Linux. */
        (l, _) if domain == Domain::Unix && l != SOL_SOCKET => {
            return errno::fail(errno::EOPNOTSUPP)
        }
        (l, n) if super::more::known(l, n) && !(l == IPPROTO_TCP && proto == Proto::Dgram) => {
            return super::more::set(&mut o.more, proto, l, n, int)
        }
        (SOL_SOCKET, SO_REUSEADDR) => o.reuseaddr = int != 0,
        (SOL_SOCKET, SO_REUSEPORT) => o.reuseport = int != 0,
        (SOL_SOCKET, SO_KEEPALIVE) => o.keepalive = int != 0,
        (SOL_SOCKET, SO_BROADCAST) => o.broadcast = int != 0,
        (SOL_SOCKET, SO_RCVBUF) => o.rcvbuf = (int.min(BUF_MAX) * 2).max(RCVBUF_MIN),
        (SOL_SOCKET, SO_SNDBUF) => o.sndbuf = (int.min(BUF_MAX) * 2).max(SNDBUF_MIN),
        (SOL_SOCKET, SO_LINGER) if len >= 8 => {
            o.linger = (u32::from(int != 0), u32::from_le_bytes([raw[4], raw[5], raw[6], raw[7]]))
        }
        (SOL_SOCKET, SO_LINGER) => return errno::fail(errno::EINVAL),
        (SOL_SOCKET, SO_RCVTIMEO) => return timeo(&mut o.rcvtimeo, word(0), word(8)),
        (SOL_SOCKET, SO_SNDTIMEO) => return timeo(&mut o.sndtimeo, word(0), word(8)),
        (IPPROTO_TCP, _) if proto == Proto::Dgram => return errno::fail(errno::ENOPROTOOPT),
        (IPPROTO_TCP, TCP_NODELAY) => o.nodelay = int != 0,
        (IPPROTO_TCP, TCP_KEEPIDLE) => return keep(&mut o.keepidle, int, KEEP_MAX),
        (IPPROTO_TCP, TCP_KEEPINTVL) => return keep(&mut o.keepintvl, int, KEEP_MAX),
        (IPPROTO_TCP, TCP_KEEPCNT) => return keep(&mut o.keepcnt, int, KEEPCNT_MAX),
        /* An AF_INET socket has no IPv6 options on Linux either. */
        (IPPROTO_IPV6, _) => return errno::fail(errno::ENOPROTOOPT),
        _ => return super::get::unknown("setsockopt", level, name),
    }
    errno::ok(0)
}
