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

//! The shared ids, recomputed here from the plan's formulas with the tags
//! hashed from their labels, not taken from the config's constants.

use nonos_marketplace_abi::release_signing_bytes;
use nonos_marketplace_abi::{bond_id, listing_id, release_hash, release_id};

use super::fixtures::sold;
use super::hosts::{keccak, Host};

fn encode(words: &[[u8; 32]]) -> Vec<u8> {
    words.iter().flatten().copied().collect()
}

#[test]
fn listing_and_release_ids_follow_plan_23() {
    let key = [0x42; 32];
    let tag = keccak(b"NONOS.nox.listing.v1");
    let want = keccak(&encode(&[tag, key, keccak(b"linux.qwenchat")]));
    let listing = listing_id(&Host, &key, "linux.qwenchat");
    assert_eq!(listing, want);

    let tag = keccak(b"NONOS.nox.release.v1");
    let want = keccak(&encode(&[tag, listing, keccak(b"qwenchat@1.0")]));
    assert_eq!(release_id(&Host, &listing, "qwenchat@1.0"), want);
    assert_ne!(listing_id(&Host, &[0x43; 32], "linux.qwenchat"), listing);
}

#[test]
fn a_bond_id_pads_the_payee_into_its_word() {
    let (payee, key) = ([0x5a; 20], [0x42; 32]);
    let mut who = [0u8; 32];
    who[12..].copy_from_slice(&payee);
    let want = keccak(&encode(&[keccak(b"NONOS.nox.bond.v1"), who, key]));
    assert_eq!(bond_id(&Host, &payee, &key), want);
}

#[test]
fn a_release_hash_exists_only_for_schema_3() {
    let mut rel = sold().entries[0].releases[0].clone();
    let want = keccak(&release_signing_bytes(&rel));
    assert_eq!(release_hash(&Host, &rel), Some(want));
    rel.ext = None;
    assert_eq!(release_hash(&Host, &rel), None);
}
