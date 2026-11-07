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

//! Which socket a listen or an accept may act on. Pure, so the host proofs
//! hold it.
//!
//! net.tcp answers net.sockets alone for a listen, and keys every handle by
//! net.sockets' pid, so it cannot tell one app's handle from another's: the
//! split is held here. A mixnet socket's transport handle is net.nym's, and
//! a connected stream's is a connection's; either could be any number,
//! one that net.tcp gave a listener among them, so an accept takes only from
//! a stream socket this app put into listening.

use crate::protocol::{E_ALREADY_BOUND, E_NOT_BOUND, E_NO_TRANSPORT, E_TABLE_FULL};
use crate::sockets::{Kind, Socket};

/// The port a listen opens on `sock`: a bound stream socket that holds no
/// transport yet, neither a connection nor a listener.
pub fn listen_port(sock: &Socket) -> Result<u16, u16> {
    let local = sock.local.filter(|_| sock.kind == Kind::Stream).ok_or(E_NOT_BOUND)?;
    if sock.listening || sock.transport_handle != 0 {
        return Err(E_ALREADY_BOUND);
    }
    Ok(local.port)
}

/// The net.tcp listener an accept on `sock` takes from: only a stream socket
/// that is listening.
pub fn accept_parent(sock: &Socket) -> Result<u32, u16> {
    if sock.kind != Kind::Stream || !sock.listening || sock.transport_handle == 0 {
        return Err(E_NOT_BOUND);
    }
    Ok(sock.transport_handle)
}

/// net.tcp's ACCEPT answer when no connection has finished its handshake.
pub const TCP_RX_EMPTY: u16 = 11;
/// net.tcp's ACCEPT answer when one has and net.sockets holds every
/// connection handle it may; the connection stays queued.
pub const TCP_NO_ROOM: u16 = 16;

/// How an accept net.tcp refused is answered, and whether the listener is
/// gone. Nothing waiting is "no transport", as an accept has always been
/// answered with nothing pending, and no room is "table full"; both leave the
/// listener. Anything else (no such listener, as net.core answers once the
/// route left Direct and its reap closed every listener, or net.tcp not
/// answering) is the listener gone: the socket no longer listens, and its
/// owner is told so and may listen again, rather than asking a dead listener
/// for good.
pub fn accept_refused(tcp_errno: u16) -> (u16, bool) {
    match tcp_errno {
        TCP_RX_EMPTY => (E_NO_TRANSPORT, false),
        TCP_NO_ROOM => (E_TABLE_FULL, false),
        _ => (E_NOT_BOUND, true),
    }
}
