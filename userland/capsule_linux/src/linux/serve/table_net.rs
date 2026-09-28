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

//! Calls that name a socket.

use crate::linux::abi::nr;
use crate::linux::guest::Guest;
use crate::linux::net;
use crate::linux::unix::{self, is_unix};

/// Every socket call. The ones that can wait reach here only when they
/// cannot: a socket's own calls are routed to `waits_sock` first, and the
/// rest are a resolver's, a display socket's, or not a socket at all.
pub fn net_ops(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Option<u64> {
    let _ = tid;
    Some(match nr {
        nr::SOCKET => net::socket(guest, a[0], a[1], a[2]),
        nr::SOCKETPAIR => net::socketpair(guest, a[0], a[1], a[2], a[3]),
        nr::CONNECT if is_unix(guest, a[0]) => unix::connect(guest, a[0], a[1], a[2]),
        nr::CONNECT => net::connect(guest, a[0], a[1], a[2]),
        nr::BIND => net::bind(guest, a[0], a[1], a[2]),
        nr::LISTEN => net::listen(guest, a[0], a[1]),
        nr::ACCEPT => net::accept4(guest, a[0], a[1], a[2], 0),
        nr::ACCEPT4 => net::accept4(guest, a[0], a[1], a[2], a[3]),
        nr::GETSOCKNAME => net::getsockname(guest, a[0], a[1], a[2]),
        nr::GETPEERNAME => net::getpeername(guest, a[0], a[1], a[2]),
        nr::SETSOCKOPT => net::setsockopt(guest, a[0], a[1], a[2], a[3], a[4]),
        nr::GETSOCKOPT => net::getsockopt(guest, a[0], a[1], a[2], a[3], a[4]),
        nr::SENDTO => net::sendto(guest, a[0], a[1], a[2], a[3], a[4], a[5]),
        nr::RECVFROM => net::recvfrom(guest, a[0], a[1], a[2], a[3], a[4], a[5]),
        nr::SHUTDOWN => net::shutdown(guest, a[0], a[1]),
        nr::SENDMSG if is_unix(guest, a[0]) => unix::sendmsg(guest, a[0], a[1]),
        nr::RECVMSG if is_unix(guest, a[0]) => unix::recvmsg(guest, a[0], a[1]),
        nr::SENDMSG => net::sendmsg(guest, a[0], a[1], a[2], 0),
        nr::RECVMSG => net::recvmsg(guest, a[0], a[1], a[2], 0),
        nr::SENDMMSG => net::sendmmsg(guest, a, 0),
        nr::RECVMMSG => net::recvmmsg(guest, a, 0),
        _ => return None,
    })
}
