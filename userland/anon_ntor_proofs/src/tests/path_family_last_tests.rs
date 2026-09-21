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
//! Path families, the rest.

use crate::path::{candidates, Flags, Position, Relay, Taken, Weights};
use alloc::vec::Vec;

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
fn the_candidate_set_drops_every_relay_sharing_the_taken_network() {
    let pool: Vec<Relay> = alloc::vec![
        relay([51, 13, 0, 1], 1),
        relay([51, 13, 99, 2], 2),
        relay([95, 216, 0, 3], 3),
        relay([5, 161, 0, 4], 4),
    ];
    let on_path = [taken(&pool[0])];
    let (chosen, weights) = candidates(&pool, &flat(), Position::Middle, &on_path);
    assert_eq!(chosen.len(), 2, "both 51.13 relays are out, the other two stay");
    assert_eq!(weights.len(), chosen.len(), "the two vectors stay index aligned");
    assert!(chosen.iter().all(|r| r.address[0] != 51), "nothing from the taken network survives");
}
