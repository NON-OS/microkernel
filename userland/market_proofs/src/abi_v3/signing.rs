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

//! The publisher's signing bytes: v3 covers every field schema 3 adds, and a
//! v2 signature can never be passed off as v3.

use nonos_marketplace_abi::{commerce_hash_input, release_signing_bytes, COMMERCE_DOMAIN};

use super::fixtures::sold;

#[test]
fn a_schema_3_release_signs_under_v3_and_a_schema_2_one_under_v2() {
    let mut rel = sold().entries[0].releases[0].clone();
    assert_eq!(&release_signing_bytes(&rel)[4..32], b"NONOS.marketplace.release.v3");
    rel.ext = None;
    assert_eq!(&release_signing_bytes(&rel)[4..32], b"NONOS.marketplace.release.v2");
}

#[test]
fn every_schema_3_field_is_under_the_signature() {
    let base = sold().entries[0].releases[0].clone();
    let signed = release_signing_bytes(&base);
    let edits: [fn(&mut nonos_marketplace_abi::ReleaseExt); 5] = [
        |x| x.commerce_hash[0] ^= 1,
        |x| x.guest_policy[255] ^= 1,
        |x| x.chunk_root[31] ^= 1,
        |x| x.content_len += 1,
        |x| x.proof_anchors.pop().map(|_| ()).unwrap_or(()),
    ];
    for edit in edits {
        let mut rel = base.clone();
        edit(rel.ext.as_mut().unwrap());
        assert_ne!(release_signing_bytes(&rel), signed);
    }
}

#[test]
fn the_commerce_hash_input_is_the_domain_then_the_block() {
    let c = sold().entries[0].commerce.clone().unwrap();
    let input = commerce_hash_input(&c);
    assert!(input.starts_with(COMMERCE_DOMAIN));
    let mut cheaper = c.clone();
    cheaper.unit_price -= 1;
    assert_ne!(commerce_hash_input(&cheaper), input);
}
