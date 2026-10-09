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

//! Letting go of what a socket holds below this service: its TCP stream,
//! its UDP port, or its mixnet connection and the bytes kept for it.

use crate::clients::{nym, tcp, udp};
use crate::protocol::E_NO_TRANSPORT;
use crate::server::handlers::mixnet_residual;
use crate::sockets::{Kind, Socket};
use crate::state;

/// Release `sock`'s transport. An error leaves the socket's entry for the
/// caller to keep, as a close that could not reach its transport does.
pub fn release(sock: &Socket) -> Result<(), u16> {
    let key = sock.key;
    if sock.kind == Kind::Stream && sock.transport_handle != 0 {
        if tcp::close(state::tcp(), sock.transport_handle).is_err() {
            return Err(E_NO_TRANSPORT);
        }
    }
    if sock.kind == Kind::Datagram {
        if let Some(local) = sock.local {
            if udp::unbind(state::udp(), local.port).is_err() {
                return Err(E_NO_TRANSPORT);
            }
        }
    }
    if sock.kind == Kind::Mixnet && sock.transport_handle != 0 {
        // Drop anything still held for this socket first, so a later socket
        // handed the same handle does not read bytes meant for this one.
        mixnet_residual::release(key);
        if nym::close(state::nym(), sock.transport_handle).is_err() {
            return Err(E_NO_TRANSPORT);
        }
    }
    super::recv_replay::release(key);
    Ok(())
}
