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

//! What the market writes for OP_GET_COMMERCE is what its clients read: the
//! capsule's own encoder, parsed by market_proto, against the abi.

use nonos_market_proto::{parse_commerce, BINDING_VALID, COMMERCE_MATCHES};
use nonos_marketplace_abi::release_hash;
use nonos_marketplace_abi::{bond_id, commerce_from_bytes, commerce_hash_input, listing_id};
use nonos_marketplace_abi::{BindingVerdict, CommerceVerdict};

use super::chain_hosts::Host;
use super::commerce_encode::encode;
use super::fixtures::sold;

#[test]
fn the_reply_reads_back_field_for_field() {
    let mut entry = sold().entries[0].clone();
    let c = entry.commerce.as_mut().unwrap();
    c.bond_id = bond_id(&Host, &c.payee, &entry.publisher_pubkey);
    let block = commerce_hash_input(entry.commerce.as_ref().unwrap());
    entry.releases[0].ext.as_mut().unwrap().commerce_hash = *blake3::hash(&block).as_bytes();
    let rel = &entry.releases[0];
    let got = parse_commerce(&encode(&entry, rel, true).expect("fits")).expect("parses");
    assert_eq!(got.commerce_verdict, CommerceVerdict::Matches as u8);
    assert_eq!(got.commerce_verdict, COMMERCE_MATCHES);
    // No bond contract is deployed, so the binding cannot be checked yet.
    assert_eq!(got.binding_verdict, BindingVerdict::NoDomain as u8);
    assert_ne!(got.binding_verdict, BINDING_VALID);
    assert!(!got.trusted());
    let k = &Host;
    assert_eq!(got.listing_id, listing_id(k, &entry.publisher_pubkey, &entry.listing_id));
    assert_eq!(Some(got.release_hash), release_hash(k, rel));
    assert_eq!(got.chain_id, entry.token.chain_id);
    assert_eq!(commerce_from_bytes(&got.block).ok(), entry.commerce);
}

#[test]
fn an_unsigned_release_does_not_vouch_for_its_commerce() {
    let mut entry = sold().entries[0].clone();
    let block = commerce_hash_input(entry.commerce.as_ref().unwrap());
    entry.releases[0].ext.as_mut().unwrap().commerce_hash = *blake3::hash(&block).as_bytes();
    let got = parse_commerce(&encode(&entry, &entry.releases[0], false).unwrap()).unwrap();
    assert_eq!(got.commerce_verdict, CommerceVerdict::Mismatch as u8);
}

#[test]
fn a_schema_2_listing_has_nothing_to_sell() {
    let mut entry = sold().entries[0].clone();
    entry.commerce = None;
    entry.releases[0].ext = None;
    let got = parse_commerce(&encode(&entry, &entry.releases[0], true).unwrap()).unwrap();
    assert_eq!(got.commerce_verdict, CommerceVerdict::NotV3 as u8);
    assert_eq!(got.binding_verdict, BindingVerdict::NoCommerce as u8);
    assert_eq!((got.release_hash, got.block.len()), ([0; 32], 0));
}
