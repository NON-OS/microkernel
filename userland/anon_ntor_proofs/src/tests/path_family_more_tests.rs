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
//! Path families, continued.

use crate::path::eligible::excluded;
use crate::path::{candidates, Flags, Position, Relay, Taken, Weights};

fn relay(address: [u8; 4], id: u8) -> Relay {
    Relay {
        address,
        or_port: 9001,
        rsa_identity: [id; 20],
        ed25519_identity: [id; 32],
        ntor_onion_key: [id; 32],
        flags: Flags {
            running: true,
            valid: true,
            fast: true,
            stable: true,
            guard: true,
            exit: true,
            authority: false,
        },
        weight: 1000,
        exits_web: true,
    }
}

fn taken(relay: &Relay) -> Taken {
    Taken { identity: relay.rsa_identity, address: relay.address }
}

fn flat() -> Weights {
    Weights {
        wgg: 10_000,
        wgd: 10_000,
        wmg: 10_000,
        wmd: 10_000,
        wme: 10_000,
        wmm: 10_000,
        wee: 10_000,
        wed: 10_000,
    }
}

#[test]
fn a_relay_one_octet_outside_the_slash_16_is_allowed() {
    let first = relay([51, 13, 7, 9], 1);
    let other = relay([51, 14, 7, 9], 2);
    assert!(!excluded(&other, &[taken(&first)]), "51.14 is a different network");
}
