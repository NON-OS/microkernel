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

//! A guest's stream on the Nym mixnet: through net.socks5, as an install's
//! own stream goes on Route::Nym, by nonos_route_link's opening (the reset,
//! the greeting, a CONNECT naming the host as a DOMAINNAME, unresolved, and
//! the retries while the mixnet has not connected yet, under OPEN_MS). Each
//! stream is one of net.socks5's named streams, since this capsule is one
//! caller for every guest. Nothing here ever looks the name up.

use nonos_route_link::{Route, RouteOpening};

use super::entry::Dest;

/// How long one call to net.socks5 waits before the serve loop moves on:
/// past the 40 ms net.socks5 holds a poll while its exit is quiet, so a
/// quiet stream is answered rather than asked again.
pub const SLICE_MS: u64 = 50;

/// What a host that is not text, which no guest can hand in, is told.
const NOT_TEXT: &str = "the host name is not text";

/// The opening of a stream to `dest` on named stream `stream` of net.socks5
/// at `port`, or why none begins.
pub fn begin(port: u32, dest: &Dest, stream: u32) -> Result<RouteOpening, &'static str> {
    let host = core::str::from_utf8(&dest.host).map_err(|_| NOT_TEXT)?;
    RouteOpening::begin_on(Route::Nym(port), host, dest.port, SLICE_MS, stream)
}
