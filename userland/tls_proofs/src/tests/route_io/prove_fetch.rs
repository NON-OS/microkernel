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

//! capsule_prove_fetch's own RouteIo: over Nym it lends the handshake its
//! route's patience, so a first byte at 45 s completes; over a direct
//! socket the same silence ends the handshake.

use nonos_libc::mk_uptime_ms;
use nonos_route_link::RouteStream;

use super::peer::{direct, over_nym};
use crate::session::SessionError;
use crate::stream::connect_unauthenticated;

#[path = "../../../../capsule_prove_fetch/src/io.rs"]
mod io;

fn handshake(stream: RouteStream) -> Result<(), SessionError> {
    let mut between = || {};
    let until = mk_uptime_ms() + 600_000;
    let mut io = io::RouteIo { stream, between: &mut between, until };
    connect_unauthenticated(&mut io, b"relay").map(|_| ())
}

#[test]
fn a_first_byte_at_45_s_over_nym_completes() {
    assert_eq!(handshake(over_nym()), Ok(()));
}

#[test]
fn the_same_over_a_direct_socket_ends_the_handshake() {
    assert_eq!(handshake(direct()), Err(SessionError::Handshake));
}
