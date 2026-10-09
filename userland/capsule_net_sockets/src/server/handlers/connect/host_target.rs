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

//! Where a connect by name goes, decided before anything is asked.
//!
//! A mixnet socket's frames carry an IPv4 address and a port and nothing
//! else (`mixnet_frame/types.rs`): its exit is told an address, never a
//! name. A name could therefore reach the exit only after being resolved
//! here, through net.dns, in the clear, which named the host to the
//! resolver and to the network path while the stream itself crossed the
//! mixnet to hide it. That was every connect a Linux guest made by name.
//! A mixnet socket is refused a name instead, with E_NAME_REFUSED, and
//! net.dns is never asked for it; an address written as one goes as it
//! always did. Stream and datagram sockets resolve as before.

use super::parse_ipv4::parse_ipv4;
use crate::protocol::{E_BAD_ADDR, E_NAME_REFUSED, E_NO_DNS};

/// The address a connect by `host` goes to. `resolve` is net.dns, and it is
/// asked only for a name on a socket that is not a mixnet socket.
pub fn host_target(
    mixnet: bool,
    host: &[u8],
    resolve: impl FnOnce(&[u8]) -> Result<[u8; 4], u16>,
) -> Result<[u8; 4], u16> {
    if let Some(ip) = parse_ipv4(host) {
        return Ok(ip);
    }
    if mixnet {
        return Err(E_NAME_REFUSED);
    }
    resolve(host).map_err(lookup_status)
}

/// net.dns's answer for a failed lookup, as this service's status. net.dns
/// unanswered (the envelope's 15) or saying no upstream server answered it
/// (its E_TIMEOUT, 6) is no DNS server reachable; anything else is a name
/// with no address.
pub fn lookup_status(dns: u16) -> u16 {
    match dns {
        6 | 15 => E_NO_DNS,
        _ => E_BAD_ADDR,
    }
}
