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

use alloc::vec::Vec;

use super::fetch_at::fetch_at;
use super::lease::wait_for_lease;

/// Fetch `path` from `host` over TLS and return the response body, trying
/// each of `addresses` in order until one answers.
///
/// The addresses only say where to connect. The certificate chain is still
/// checked against `host`, and before the request is written, so an address
/// that no longer belongs to `host` fails the handshake and never sees what
/// was being asked for. No name is resolved here, so no DNS query leaves the
/// machine. Nothing about this fetch is anonymous: it happens before there is
/// a mixnet to be anonymous over. `max` bounds what the far end can make the
/// capsule allocate for one answer.
pub fn fetch_tls(
    tcp_port: u32,
    host: &str,
    addresses: &[[u8; 4]],
    path: &str,
    max: usize,
) -> Result<Vec<u8>, u16> {
    wait_for_lease()?;
    let mut last = 21u16;
    for ip in addresses {
        match fetch_at(tcp_port, *ip, host, path, max) {
            Ok(body) => return Ok(body),
            Err(code) => last = code,
        }
    }
    Err(last)
}
