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

//! Whether a relay may serve a position at all, before weight is considered.

use super::super::relay::Relay;
use super::super::weights::Position;

pub(super) fn eligible(relay: &Relay, position: Position) -> bool {
    if !relay.usable() || relay.flags.authority {
        return false;
    }
    match position {
        Position::Guard => relay.flags.guard && relay.flags.stable && relay.flags.fast,
        Position::Middle => relay.flags.fast,
        /*
         * An Exit flag says the relay exits somewhere, not that it exits to
         * where the stream is going. The published policy summary is read as
         * well, or a circuit builds all three hops and the stream is refused
         * at the last one, which costs three handshakes to learn.
         */
        Position::Exit => relay.flags.exit && relay.flags.fast && relay.exits_web,
    }
}

/*
 * Two hops in the same /16 are very often the same operator, the same rack, or
 * at least the same transit, and either of those sees both ends of the leg
 * between them. Excluding only the identity would let a path run guard and exit
 * through one datacentre and still look like three hops, which is the one thing
 * three hops is supposed to rule out. Tor draws the same line at a /16 for IPv4,
 * in `addrs_in_same_network_family`.
 */

/// True when `relay` must not be added to a path already holding `taken`.
pub fn excluded(relay: &Relay, taken: &[Taken]) -> bool {
    taken.iter().any(|t| t.identity == relay.rsa_identity || same_16(t.address, relay.address))
}

/// A hop already committed to the path being drawn.
#[derive(Clone, Copy)]
pub struct Taken {
    pub identity: [u8; 20],
    pub address: [u8; 4],
}

fn same_16(a: [u8; 4], b: [u8; 4]) -> bool {
    a[0] == b[0] && a[1] == b[1]
}
