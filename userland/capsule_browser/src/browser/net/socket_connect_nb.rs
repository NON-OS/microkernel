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

//! Starting a connection without waiting for it.

use super::constants::{OP_CONNECT_NB, SOCKETS_MAGIC};

/* net.sockets starts the handshake and answers at once. */
const NB_CALL_MS: u64 = 1000;

/// Ask net.sockets to start connecting `handle` to `ip`:`port`. Success
/// means the handshake has begun; `socket_poll` says when it has finished.
pub fn socket_connect_nb(sockets_port: u32, handle: u32, ip: [u8; 4], port: u16) -> Result<(), ()> {
    // An address goes through net.sockets, which is direct.
    if super::mixnet::is_proxied(handle) || !super::mixnet::direct_allowed() {
        return Err(());
    }
    let mut body = [0u8; 10];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..8].copy_from_slice(&ip);
    body[8..10].copy_from_slice(&port.to_le_bytes());
    let mut rx = [0u8; 20];
    super::call::call_t(sockets_port, SOCKETS_MAGIC, OP_CONNECT_NB, &body, &mut rx, NB_CALL_MS)?;
    Ok(())
}
