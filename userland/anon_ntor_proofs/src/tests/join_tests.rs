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
//! Join.

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
fn an_entry_takes_the_keys_from_its_own_microdescriptor() {
    let entries = alloc::vec![entry(7, [10, 0, 0, 1])];
    let relays = join(&entries, &held(&[7]));
    assert_eq!(relays.len(), 1);
    assert_eq!(relays[0].ntor_onion_key, [7u8; 32], "the key of the digest it named");
    assert_eq!(relays[0].address, [10, 0, 0, 1], "and the address from the consensus half");
}
#[test]
fn an_entry_whose_microdescriptor_is_missing_is_dropped() {
    let entries = alloc::vec![entry(1, [10, 0, 0, 1]), entry(2, [10, 0, 0, 2])];
    let relays = join(&entries, &held(&[2]));
    assert_eq!(relays.len(), 1, "carrying a zero ntor key would fail at the relay instead");
    assert_eq!(relays[0].rsa_identity, [2u8; 20]);
}
