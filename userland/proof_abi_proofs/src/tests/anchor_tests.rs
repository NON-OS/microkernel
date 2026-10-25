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

use nonos_proof_abi::{anchor, anchor_tag, day, market_scope, Kind};
use nonos_stark::hash::keccak256;

#[test]
fn the_engines_keccak_is_keccak256() {
    let empty = "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";
    let hex: String = keccak256(b"").iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(hex, empty);
    assert_eq!(anchor_tag(), keccak256(b"NONOS.proof.anchor.v1"));
}

#[test]
fn the_anchor_is_the_abi_encoding_of_six_words() {
    let (listing, id, tag, c) = ([7u8; 32], [9u8; 32], [1u64, 2, 3, 4], b"container bytes");
    let mut enc = anchor_tag().to_vec();
    enc.extend_from_slice(&[0; 31]);
    enc.push(1);
    enc.extend_from_slice(&listing);
    enc.extend_from_slice(&id);
    tag.iter().for_each(|w| enc.extend_from_slice(&w.to_le_bytes()));
    enc.extend_from_slice(&keccak256(c));
    assert_eq!(enc.len(), 192);
    let a = anchor(Kind::Isolation, &listing, &id, &tag, c);
    assert_eq!(a, keccak256(&enc));
    assert_ne!(anchor(Kind::Isolation, &[8; 32], &id, &tag, c), a, "another listing");
    assert_ne!(anchor(Kind::Delivery, &listing, &id, &tag, c), a, "another kind");
}

#[test]
fn the_market_scope_is_its_own_hash_per_listing_and_day() {
    let l = [0x33u8; 32];
    let pre = [&b"NONOS-MARKET-SCOPE-v1"[..], &l, &20_000u64.to_le_bytes()].concat();
    let w = nox_verify::attest::digest_words(blake3::hash(&pre).as_bytes());
    assert_eq!(market_scope(&l, 20_000), [w[0], w[1]]);
    assert_ne!(market_scope(&l, 20_001), market_scope(&l, 20_000), "a day apart");
    assert_ne!(market_scope(&[0x34; 32], 20_000), market_scope(&l, 20_000));
    let interactive = nonos_device_attest::scope(b"nonos.market:", 20_000).expect("scope");
    assert_ne!([interactive[0].to_u64(), interactive[1].to_u64()], market_scope(&l, 20_000));
    assert_eq!((day(86_399), day(86_400)), (0, 1));
}
