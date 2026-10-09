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

use alloc::vec;

use super::ask::{ask, Fault};
use super::constants::{OP_CONNECT_HOST, SOCKETS_MAGIC};

/// net.sockets' status for a connect by name whose lookup could not be made:
/// no DNS server is reachable (E_NO_DNS in
/// userland/capsule_net_sockets/src/protocol/errno.rs).
pub const E_NO_DNS: u16 = 16;

const CONNECT_TIMEOUT_MS: u64 = 9000;

pub fn socket_connect_host(
    sockets_port: u32,
    handle: u32,
    host: &str,
    port: u16,
) -> Result<(), Option<u16>> {
    // A name goes to net.sockets to resolve and connect, which is direct.
    // A conversation with a proxy is never dialled: its CONNECT names the
    // host for the exit.
    if super::mixnet::is_proxied(handle) || !super::mixnet::direct_allowed() {
        return Err(None);
    }
    let h = host.as_bytes();
    if h.is_empty() || h.len() > 253 {
        return Err(None);
    }
    let mut body = vec![0u8; 8 + h.len()];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..6].copy_from_slice(&port.to_le_bytes());
    body[6..8].copy_from_slice(&(h.len() as u16).to_le_bytes());
    body[8..].copy_from_slice(h);
    let mut rx = [0u8; 20];
    match ask(sockets_port, SOCKETS_MAGIC, OP_CONNECT_HOST, &body, &mut rx, CONNECT_TIMEOUT_MS) {
        Ok(_) => Ok(()),
        Err(Fault::Status(status)) => Err(Some(status)),
        Err(Fault::Lost | Fault::Garbled) => Err(None),
    }
}
