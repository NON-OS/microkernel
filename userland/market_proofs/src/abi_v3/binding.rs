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

//! A PublisherBinding signed with a real secp256k1 key, checked by the
//! market's verdict and against an EIP-712 digest built here by hand.

use nonos_marketplace_abi::{binding_digest_at, binding_verdict, binding_verdict_at, bond_id};
use nonos_marketplace_abi::{listing_id, BindingVerdict, MarketplaceEntry};
use nonos_secp256k1::{public_key_from_secret, sign};

use super::fixtures::sold;
use super::hosts::{address_of, Host};

pub(super) const BOND_AT: [u8; 20] = [0xb0; 20];

/// The fixture entry, bound by the payee whose secret is `[0x11; 32]`.
fn bound() -> MarketplaceEntry {
    let secret = [0x11; 32];
    let mut e = sold().entries[0].clone();
    let payee = address_of(&public_key_from_secret(&secret).unwrap());
    let key = e.publisher_pubkey;
    let listing = listing_id(&Host, &key, &e.listing_id);
    let c = e.commerce.as_mut().unwrap();
    c.payee = payee;
    c.bond_id = bond_id(&Host, &payee, &key);
    let digest = binding_digest_at(&Host, 1, BOND_AT, [&key, &listing, &c.bond_id]);
    let sig = sign(&secret, &digest).unwrap();
    c.binding_sig[..32].copy_from_slice(&sig.r);
    c.binding_sig[32..64].copy_from_slice(&sig.s);
    c.binding_sig[64] = sig.recovery_id + 27;
    e
}

#[test]
fn the_payees_own_signature_is_valid_and_nothing_else_is() {
    let e = bound();
    assert_eq!(binding_verdict_at(&Host, &Host, &e, BOND_AT), BindingVerdict::Valid);
    // Another bond contract, another listing, another payee: all refused.
    assert_eq!(binding_verdict_at(&Host, &Host, &e, [0xb1; 20]), BindingVerdict::WrongSigner);
    let mut renamed = e.clone();
    renamed.listing_id = "linux.other".into();
    assert_eq!(binding_verdict_at(&Host, &Host, &renamed, BOND_AT), BindingVerdict::WrongSigner);
    let mut forged = e.clone();
    forged.commerce.as_mut().unwrap().binding_sig[64] = 0;
    assert_eq!(binding_verdict_at(&Host, &Host, &forged, BOND_AT), BindingVerdict::WrongSigner);
}

#[test]
fn a_wrong_bond_or_missing_domain_or_commerce_is_named() {
    let mut e = bound();
    // No bond contract is deployed yet, so the config gives no domain.
    assert_eq!(binding_verdict(&Host, &Host, &e), BindingVerdict::NoDomain);
    e.commerce.as_mut().unwrap().bond_id[0] ^= 1;
    assert_eq!(binding_verdict(&Host, &Host, &e), BindingVerdict::WrongBond);
    e.commerce = None;
    assert_eq!(binding_verdict(&Host, &Host, &e), BindingVerdict::NoCommerce);
}
