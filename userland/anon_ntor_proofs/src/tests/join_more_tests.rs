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
//! Join, continued.

use crate::directory::consensus::Entry;
use crate::directory::microdesc::Microdesc;
use crate::manager::join;
use alloc::vec::Vec;

fn entry(digest: u8, address: [u8; 4]) -> Entry {
    Entry {
        address,
        or_port: 9001,
        rsa_identity: [digest; 20],
        microdesc_digest: [digest; 32],
        ..Entry::default()
    }
}

fn micro(digest: u8) -> ([u8; 32], Microdesc) {
    (
        [digest; 32],
        Microdesc { ntor_onion_key: [digest; 32], ed25519_identity: [digest; 32], exits_web: true },
    )
}

fn held(digests: &[u8]) -> Vec<([u8; 32], Microdesc)> {
    let mut out: Vec<([u8; 32], Microdesc)> = digests.iter().map(|d| micro(*d)).collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

#[test]
fn every_position_in_the_ordering_is_found() {
    let digests: Vec<u8> = (1..=40).collect();
    let entries: Vec<Entry> = digests.iter().map(|d| entry(*d, [10, 0, 0, *d])).collect();
    let relays = join(&entries, &held(&digests));
    assert_eq!(relays.len(), 40, "first, last and everything between");
    for (i, relay) in relays.iter().enumerate() {
        assert_eq!(relay.ntor_onion_key, [digests[i]; 32], "no entry took another's key");
    }
}
#[test]
fn the_output_follows_the_consensus_order() {
    let entries = alloc::vec![entry(9, [10, 0, 0, 9]), entry(3, [10, 0, 0, 3])];
    let relays = join(&entries, &held(&[3, 9]));
    assert_eq!(relays.len(), 2);
    assert_eq!(relays[0].rsa_identity, [9u8; 20], "entry order, not digest order");
    assert_eq!(relays[1].rsa_identity, [3u8; 20]);
}
