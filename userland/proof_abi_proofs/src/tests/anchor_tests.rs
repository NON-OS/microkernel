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

use nonos_proof_abi::{anchor, day, scope, verifier_id, Kind};
use nonos_stark::hash::keccak256;

#[test]
fn the_engines_keccak_is_keccak256() {
    let empty = "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";
    let hex: String = keccak256(b"").iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(hex, empty);
}

#[test]
fn the_anchor_is_the_packed_encoding() {
    let (id, tag, c) = ([9u8; 32], [1u64, 2, 3, 4], b"container bytes");
    let mut packed = b"NONOS-PROOF-ANCHOR-v1".to_vec();
    packed.push(1);
    packed.extend_from_slice(&id);
    tag.iter().for_each(|w| packed.extend_from_slice(&w.to_le_bytes()));
    packed.extend_from_slice(&keccak256(c));
    assert_eq!(anchor(Kind::Isolation, &id, &tag, c), keccak256(&packed));
    assert_ne!(anchor(Kind::Delivery, &id, &tag, c), anchor(Kind::Isolation, &id, &tag, c));
}

#[test]
fn the_scope_is_the_device_statements_own() {
    let l = [0x33u8; 32];
    let id = verifier_id(&l);
    assert_eq!(&id[..13], b"nonos.market:");
    let e = nonos_device_attest::scope(&id, 20_000).expect("scope");
    assert_eq!(scope(&l, 20_000), Some([e[0].to_u64(), e[1].to_u64()]));
    assert_ne!(scope(&l, 20_001), scope(&l, 20_000), "a day apart");
    assert_ne!(scope(&[0x34; 32], 20_000), scope(&l, 20_000), "a listing apart");
    assert_eq!((day(86_399), day(86_400)), (0, 1));
}
