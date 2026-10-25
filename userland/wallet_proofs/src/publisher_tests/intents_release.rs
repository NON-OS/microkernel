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

//! A release's registry and anchor calls from the entry: ids as
//! marketplace_abi derives them, the registry's own listingId recomputed
//! from publish's words, a schema 2 release refused, and one kind 1 anchor
//! per proof_anchors word (Q5).

use nonos_marketplace_abi::{listing_id, release_hash, release_id, Keccak, SettleMode};

use super::fixture::{entry, K, KEY, ME};
use crate::intent::{anchors_of, encode, listing_of, publish_of, withdraw_of, Publisher, Why};

pub(super) const URI: &str = "https://example.org/r/qwen-small@1";

#[test]
fn publish_carries_the_entrys_ids() {
    let e = entry(SettleMode::Escrow);
    let listing = listing_id(&K, &KEY, "linux.qwen-small");
    let calls = publish_of(&K, &e, &ME, 1, URI).unwrap();
    let Publisher::Publish {
        publisher_key,
        app_id_hash,
        release_id: r,
        release_hash: h,
        bond_id,
        uri,
    } = &calls[0]
    else {
        panic!("{calls:?}")
    };
    assert_eq!(calls.len(), 1);
    assert_eq!((*publisher_key, *app_id_hash), (KEY, K.keccak256(b"linux.qwen-small")));
    assert_eq!(listing_of(&K, publisher_key, app_id_hash), listing, "the registry's recompute");
    assert_eq!(*r, release_id(&K, &listing, "qwen-small@2"));
    assert_eq!(Some(*h), release_hash(&K, &e.releases[1]));
    assert_eq!((*bond_id, uri.as_slice()), (e.commerce.unwrap().bond_id, URI.as_bytes()));
    assert!(encode(&calls[0], |_| Some([0xA5; 20])).is_some(), "the encoder takes it");
    let w = withdraw_of(&K, &entry(SettleMode::Escrow), &ME, 1).unwrap();
    assert_eq!(w, vec![Publisher::WithdrawRelease { listing_id: listing, release_id: *r }]);
}

#[test]
fn a_schema_2_release_is_refused_and_each_anchor_is_one_call() {
    let mut e = entry(SettleMode::Escrow);
    let listing_id = listing_id(&K, &KEY, "linux.qwen-small");
    assert_eq!(anchors_of(&K, &e, &ME, 0), Ok(vec![]), "no anchors, no calls");
    e.releases[0].ext.as_mut().unwrap().proof_anchors = vec![[0xA1; 32], [0xA2; 32]];
    let one = |anchor| Publisher::Anchor { anchor, kind: 1, listing_id };
    assert_eq!(anchors_of(&K, &e, &ME, 0), Ok(vec![one([0xA1; 32]), one([0xA2; 32])]));
    e.releases[0].ext = None;
    assert_eq!(publish_of(&K, &e, &ME, 0, URI), Err(Why::Schema2));
    assert_eq!(anchors_of(&K, &e, &ME, 0), Err(Why::Schema2));
    assert!(withdraw_of(&K, &e, &ME, 0).is_ok(), "nothing in withdraw reads the release body");
}
