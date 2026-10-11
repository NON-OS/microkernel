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

//! What an open asks net.anon for, and the shape of the reply it gets.

use alloc::format;
use alloc::vec::Vec;

/// The longest legal domain name, as host_body.rs bounds a name for
/// net.sockets.
const MAX_HOST: usize = 253;

/// One reply: its status and body, or None when none came or it was not a
/// reply (anon_wire.rs).
pub type Reply<'a> = Option<(u16, &'a [u8])>;

/// An open's body: the port, then the host. A host a BEGIN cannot carry as
/// written (empty, too long, or holding the ':' or NUL that end its part of
/// "host:port") is None.
pub fn open_body(host: &[u8], port: u16) -> Option<Vec<u8>> {
    if host.is_empty() || host.len() > MAX_HOST || host.iter().any(|&b| b == b':' || b == 0) {
        return None;
    }
    let mut body = Vec::with_capacity(2 + host.len());
    body.extend_from_slice(&port.to_le_bytes());
    body.extend_from_slice(host);
    Some(body)
}

/// A literal address as net.anon is given it: the dotted quad, so the exit
/// connects to it and nothing here looks a name up.
pub fn dotted(ip: [u8; 4]) -> Vec<u8> {
    let [a, b, c, d] = ip;
    format!("{a}.{b}.{c}.{d}").into_bytes()
}
