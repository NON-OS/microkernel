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

//! A sealed region's key and access hash, and an attestation link's nonce,
//! signature half and quantum proof: each drawn; a refused draw adds no
//! region and no link.

use super::super::types::CapabilityEngine;
use super::engine;
use crate::security::policy::capability::isolation::IsolationLevel;
use crate::stand_in::rng;

fn chamber(e: &CapabilityEngine, level: IsolationLevel) -> u64 {
    let id = e.create_isolation_chamber(level, &[]).expect("chamber");
    rng::fail_from(None);
    id
}

#[test]
fn no_region_is_sealed_when_either_draw_refuses() {
    let e = engine();
    let id = chamber(&e, IsolationLevel::Basic);
    for (from, only) in rng::cases(2) {
        rng::fail_at(Some(from), only);
        assert_eq!(e.seal_memory_region(id, 0x10_0000, 4096, 3), Err("no randomness"));
        assert!(e.chambers.read()[&id].sealed_memory_regions.read().is_empty());
    }
    rng::fail_from(None);
    e.seal_memory_region(id, 0x10_0000, 4096, 3).expect("sealed");
    let all = e.chambers.read();
    let r = &all[&id].sealed_memory_regions.read()[0];
    assert_eq!(rng::given(), [r.encryption_key.to_vec(), r.access_pattern_hash.to_vec()]);
    assert_eq!(rng::weak(), 0);
}

#[test]
fn no_link_is_added_when_any_draw_refuses() {
    let e = engine();
    let id = chamber(&e, IsolationLevel::QuantumSecure);
    for (from, only) in rng::cases(3) {
        rng::fail_at(Some(from), only);
        assert_eq!(e.create_attestation_chain(id, [1; 32], &[]), Err("no randomness"));
        assert!(e.chambers.read()[&id].attestation_chain.read().is_empty());
    }
    rng::fail_from(None);
    e.create_attestation_chain(id, [1; 32], &[]).expect("linked");
    let all = e.chambers.read();
    let link = &all[&id].attestation_chain.read()[0];
    let proof = link.quantum_proof.expect("proof").to_vec();
    let made = [link.nonce.to_vec(), link.signature[..32].to_vec(), proof];
    assert_eq!(rng::given(), made);
    assert_eq!(&link.signature[32..], &e.signing_key);
}
