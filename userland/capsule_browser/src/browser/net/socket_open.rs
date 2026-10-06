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

use super::constants::{OP_SOCKET, SOCKETS_MAGIC, SOCKET_FAMILY_IP4, SOCKET_KIND_STREAM};
use super::mixnet::{Way, PROXIED};

/// A socket for a connection that leaves `way`: a conversation with the
/// proxy that carries it, or a socket of net.sockets for a direct one. A way
/// that is refused opens nothing, and nothing opens direct unless the page's
/// network is Direct.
pub fn socket_open(sockets_port: u32, way: Way) -> Result<u32, ()> {
    match way {
        Way::Proxy { port, .. } => return super::mixnet::open(port),
        Way::Refused(_) => return Err(()),
        Way::Direct if !super::mixnet::direct_allowed() => return Err(()),
        Way::Direct => {}
    }
    let mut body = [0u8; 4];
    let mut rx = [0u8; 32];
    body[0..2].copy_from_slice(&SOCKET_FAMILY_IP4.to_le_bytes());
    body[2..4].copy_from_slice(&SOCKET_KIND_STREAM.to_le_bytes());
    let n = super::call::call(sockets_port, SOCKETS_MAGIC, OP_SOCKET, &body, &mut rx)?;
    if n < 24 {
        return Err(());
    }
    let handle = u32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]);
    /* A handle with the proxied bit would be read as a conversation with a
     * proxy; net.sockets counts from 1 and never gets there, and if it ever
     * did the socket is handed back rather than mistaken. */
    if handle & PROXIED != 0 {
        let _ = super::socket_close::close_direct(sockets_port, handle);
        return Err(());
    }
    super::recv_seq::forget(handle);
    Ok(handle)
}
