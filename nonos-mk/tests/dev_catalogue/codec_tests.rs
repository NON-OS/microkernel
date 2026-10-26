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

use nonos_marketplace_abi::{bond_id, decode_index, encode_index, Commerce, Keccak, SCHEMA_V4};
use nonos_nox_config::UNITS;
use serde_json::Value;
use sha3::{Digest, Keccak256};

use crate::image_tests::dev_signed;
use crate::signed::SOURCE;

struct Host;

impl Keccak for Host {
    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        Keccak256::digest(data).into()
    }
}

/// What the source says, against what the decoder read back.
fn same(c: &Commerce, s: &Value, publisher: &[u8; 32]) {
    let text = |k: &str| s[k].as_str().expect("a string").trim_start_matches("0x").to_owned();
    assert_eq!(hex::encode(c.payee), text("payee"));
    assert_eq!(c.bond_id, bond_id(&Host, &c.payee, publisher), "the bond is the payee's");
    assert_eq!(Some(&c.unit), UNITS.iter().find(|(l, _)| *l == text("unit")).map(|(_, w)| w));
    assert_eq!(c.unit_price.to_string(), text("unit_price"));
    assert_eq!(c.min_charge.to_string(), text("min_charge"));
    assert_eq!(format!("{:?}", c.settle_mode).to_lowercase(), text("settle_mode"));
    assert_eq!(hex::encode(c.binding_sig), text("binding_sig"));
    let reqs = s["proof_reqs"].as_array().expect("proof_reqs");
    assert_eq!(c.proof_reqs.len(), reqs.len());
    for (r, want) in c.proof_reqs.iter().zip(reqs) {
        assert_eq!(Some(u64::from(r.kind)), want["kind"].as_u64());
        assert_eq!(Some(hex::encode(r.statement).as_str()), want["statement"].as_str());
    }
}

#[test]
fn the_catalogue_round_trips_with_its_commerce() {
    let source: Value = serde_json::from_str(SOURCE).expect("parses");
    let blob = dev_signed("codec-round-trip");
    let index = decode_index(&blob).expect("decodes").index;
    assert_eq!(index.schema_version, SCHEMA_V4);
    let listed = source["entries"].as_array().expect("entries");
    assert_eq!(index.entries.len(), listed.len());
    for (e, s) in index.entries.iter().zip(listed) {
        assert_eq!(Some(e.listing_id.as_str()), s["listing_id"].as_str());
        same(e.commerce.as_ref().expect("commerce"), &s["commerce"], &e.publisher_pubkey);
        let ext = e.releases.iter().map(|r| r.ext.as_ref().expect("schema 3 fields"));
        assert!(ext.into_iter().all(|x| x.commerce_hash != [0; 32]), "signed under its price");
    }
    // Encoding what was read gives the same bytes, signature and all.
    assert_eq!(encode_index(&index).expect("encodes").blob, blob);
}
