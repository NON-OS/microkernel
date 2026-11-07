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

//! After each poll of the stack: a listening socket that took a SYN moves to
//! its listener's queue and a fresh socket listens on the port in its place,
//! so the next client is not reset while the first is still being accepted.
//! smoltcp keeps a listening socket off every segment that carries an ACK,
//! so the queued connections and the fresh listener share the port.

use alloc::vec::Vec;

use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::tcp::{Socket, State};

use super::table::Listener;

/// A queued connection ACCEPT may hand out: its handshake is done.
pub fn ready(state: State) -> bool {
    matches!(state, State::Established | State::CloseWait)
}

/// One listener's step. `fresh` makes the socket that listens next; the
/// handles returned are queued connections that closed before any ACCEPT,
/// or a half-open one given way, already out of the listener, for the
/// caller to remove from the set.
pub fn arm<const B: usize>(
    l: &mut Listener<SocketHandle, B>,
    sockets: &mut SocketSet<'static>,
    fresh: impl FnOnce() -> Socket<'static>,
) -> Vec<SocketHandle> {
    let mut closed = l.drop_queued(|h| sockets.get::<Socket>(h).state() != State::Closed);
    let port = l.port;
    let state = sockets.get::<Socket>(l.socket).state();
    // A SYN whose sender never answers holds a socket in SYN-RECEIVED until
    // its timeout. When the backlog is full the oldest such one gives way,
    // so SYNs from addresses that never finish cannot keep clients out.
    if !matches!(state, State::Listen | State::Closed) && !l.has_room() {
        closed.extend(l.take(|h| sockets.get::<Socket>(h).state() == State::SynReceived));
    }
    let sock = sockets.get_mut::<Socket>(l.socket);
    match state {
        State::Listen => {}
        // A handshake the peer gave up on: listen again on the same socket.
        State::Closed => {
            let _ = sock.listen(port);
        }
        _ if l.has_room() => {
            let mut next = fresh();
            if next.listen(port).is_ok() {
                let taken = l.socket;
                l.socket = sockets.add(next);
                l.push(taken);
            }
        }
        // A full backlog leaves the connection on the listening socket until
        // an ACCEPT makes room; until then a new SYN is refused with a reset.
        _ => {}
    }
    closed
}
