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

//! A name's address, asked of net.dns once and remembered.
//!
//! net.sockets can resolve and connect in one call, but that call blocks the
//! window for as long as the handshake takes. Knowing the address first lets
//! the connect be started and polled instead.

use super::ask::{ask, Fault};
use super::constants::{DNS_MAGIC, DNS_SERVICE, OP_RESOLVE_A};
use super::resolve_cache::{cached, remember};
use crate::browser::fetch::Resolved;

/* net.dns's E_TIMEOUT: no upstream server answered it within its deadline. */
const NO_UPSTREAM: u16 = 6;

/* One RESOLVE_A answers in a round trip or two; this bounds a slow one. */
const RESOLVE_MS: u64 = 2000;

/// Ask net.dns for `host`. `Unavailable` when it cannot be asked or does not
/// answer in time, so the caller can let net.sockets resolve instead.
pub fn resolve(host: &str) -> Resolved {
    // net.dns asks in the clear; only a page whose network is Direct is
    // resolved that way. Unavailable sends the caller to net.sockets, which
    // refuses too.
    if !super::mixnet::direct_allowed() {
        return Resolved::Unavailable;
    }
    if let Some(ip) = cached(host) {
        return Resolved::Ip(ip);
    }
    let port = super::lookup(DNS_SERVICE);
    if port == 0 || host.is_empty() || host.len() > 253 {
        return Resolved::Unavailable;
    }
    let mut rx = [0u8; 24];
    match ask(port, DNS_MAGIC, OP_RESOLVE_A, host.as_bytes(), &mut rx, RESOLVE_MS) {
        Ok(n) if n >= 24 => {
            let ip = [rx[20], rx[21], rx[22], rx[23]];
            remember(host, ip);
            Resolved::Ip(ip)
        }
        /* No upstream server answered net.dns (its E_TIMEOUT): no name will
         * resolve, so it is said as the network, not the spelling. */
        Err(Fault::Status(NO_UPSTREAM)) => Resolved::NoDns,
        /* A name that does not exist, or a resolver that failed for it. */
        Ok(_) | Err(Fault::Status(_)) => Resolved::Unknown,
        Err(Fault::Lost | Fault::Garbled) => Resolved::Unavailable,
    }
}
