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

//! A ReleaseAttest signed with a real secp256k1 key, checked by the market's
//! verdict, and its six words built here by hand.

use nonos_marketplace_abi::{attest_verdict, attest_verdict_at, attest_words};
use nonos_marketplace_abi::{listing_id, release_id, AttestVerdict, SettleMode};

use super::attest_fixture::attested;
use super::binding::BOND_AT;
use super::hosts::Host;

#[test]
fn the_words_pin_the_chunk_tree_in_escrow_and_the_package_hash_otherwise() {
    let (e, rel) = attested(SettleMode::Escrow);
    let c = e.commerce.clone().unwrap();
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    let x = rel.ext.clone().unwrap();
    let mut len = [0u8; 32];
    len[24..].copy_from_slice(&x.content_len.to_be_bytes());
    let release = release_id(&Host, &listing, &rel.release_id);
    let escrow = [listing, release, c.bond_id, x.chunk_root, len, [0; 32]];
    assert_eq!(attest_words(&Host, &e, &c, &rel), escrow);
    let voucher = nonos_marketplace_abi::Commerce { settle_mode: SettleMode::Voucher, ..c };
    let plain = [listing, release, voucher.bond_id, [0; 32], [0; 32], rel.package_hash];
    assert_eq!(attest_words(&Host, &e, &voucher, &rel), plain);
}

#[test]
fn only_the_payees_attest_over_these_bytes_is_valid() {
    for mode in [SettleMode::Escrow, SettleMode::Voucher] {
        let (e, rel) = attested(mode);
        assert_eq!(attest_verdict_at(&Host, &Host, &e, &rel, BOND_AT), AttestVerdict::Valid);
        assert_eq!(
            attest_verdict_at(&Host, &Host, &e, &rel, [0xb1; 20]),
            AttestVerdict::WrongSigner
        );
        let mut moved = rel.clone();
        match mode {
            SettleMode::Escrow => moved.ext.as_mut().unwrap().content_len += 1,
            _ => moved.package_hash[0] ^= 1,
        }
        assert_eq!(
            attest_verdict_at(&Host, &Host, &e, &moved, BOND_AT),
            AttestVerdict::WrongSigner
        );
        // No bond contract is deployed yet, so the config gives no domain.
        assert_eq!(attest_verdict(&Host, &Host, &e, &rel), AttestVerdict::NoDomain);
    }
    let (mut e, rel) = attested(SettleMode::Voucher);
    e.commerce = None;
    assert_eq!(attest_verdict(&Host, &Host, &e, &rel), AttestVerdict::NotV3);
}
