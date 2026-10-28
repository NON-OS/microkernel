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

//! The exit of a circuit drawn for a port: it takes the port, and the draw is
//! otherwise the one every exit has.

use crate::directory::microdesc::summary::summary;
use crate::path::{choose, choose_exit, through, Exits, Ports, Position, Taken};

use crate::tests::path_pool::{flat, pool, relay};

/// Relays 1 to 16 as exits: the even ones let 18081 out, the odd ones the
/// web and nothing else (Tor's reduced exit policy, cut short).
pub fn exits() -> Exits {
    let line = |i: u8| [&b"accept 80,443,18081"[..], b"accept 80,443"][usize::from(i) % 2];
    let held = (1..=16u8).map(|i| ([i; 20], summary(line(i)).unwrap())).collect();
    Exits { ports: Ports::new(held), ..Exits::default() }
}

pub fn dice(seed: u64) -> impl FnMut() -> Option<u64> {
    let mut n = seed;
    move || {
        n = n.wrapping_mul(2862933555777941757).wrapping_add(3037000493);
        Some(n)
    }
}

#[test]
fn a_circuit_for_a_port_ends_at_an_exit_that_takes_it() {
    let (relays, exits) = (pool(), exits());
    for seed in 0..500u64 {
        let path = through(&relays[0], &relays, &flat(), |e| exits.takes(e, 18081), dice(seed));
        let path = path.expect("six exits take the port");
        assert!(exits.takes(&path[2], 18081));
        assert_eq!(path[2].rsa_identity[0] % 2, 0);
    }
}

#[test]
fn for_the_web_ports_the_draw_is_the_one_it_always_was() {
    let (mut relays, exits) = (pool(), exits());
    relays[3].exits_web = false;
    let taken = [Taken { identity: [5; 20], address: [10, 5, 0, 1] }];
    for roll in 0..5_000u64 {
        let before = choose(&relays, &flat(), Position::Exit, &taken, roll).map(|r| r.rsa_identity);
        let web = choose_exit(&relays, &flat(), &taken, |e| exits.takes(e, 443), roll);
        assert_eq!(web.map(|r| r.rsa_identity), before, "roll {roll}");
    }
}

#[test]
fn identity_and_slash_16_keep_an_exit_for_a_port_off_the_path() {
    let (mut relays, exits) = (pool(), exits());
    relays.push(relay([10, 4, 9, 9], 14));
    let guard = relays[3].clone();
    for seed in 0..500u64 {
        let path = through(&guard, &relays, &flat(), |e| exits.takes(e, 18081), dice(seed));
        let path = path.unwrap();
        assert!(path[1..].iter().all(|r| r.address[..2] != [10, 4]), "not in the guard's /16");
        assert_ne!(path[2].rsa_identity, path[1].rsa_identity);
    }
}
