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

//! The body of a connect-by-host request, in the one layout net.sockets
//! parses (capsule_net_sockets connect/parse_host.rs): handle u32, port u16,
//! host length u16, then the host, all little-endian.

use alloc::vec::Vec;

/// The longest legal domain name, and net.sockets' own bound.
const MAX_HOST: usize = 253;

pub fn host_body(handle: u32, port: u16, host: &[u8]) -> Option<Vec<u8>> {
    if host.is_empty() || host.len() > MAX_HOST {
        return None;
    }
    let mut body = Vec::with_capacity(8 + host.len());
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&port.to_le_bytes());
    body.extend_from_slice(&(host.len() as u16).to_le_bytes());
    body.extend_from_slice(host);
    Some(body)
}
