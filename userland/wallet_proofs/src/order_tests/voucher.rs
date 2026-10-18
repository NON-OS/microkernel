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

//! A voucher listing's channel (v1.4.38): the market's terms, and OPEN
//! 12/0's request once the person sets the deposit and the expiry.

use nonos_market_proto::{BondCode, ReleaseEscrow};
use nonos_marketplace_abi::SettleMode;
use nonos_payment_proto::{parse_open, OpenStart};

use super::fixture::{buy, NOW, REFUND_TO};
use super::read_fixture::{commerce, escrow, ready, CHAIN};
use crate::order::read::{read_named, Named};
use crate::order::voucher::Voucher;

pub(super) fn voucher() -> Voucher {
    let c = commerce(SettleMode::Voucher);
    let bare = ReleaseEscrow { chunk_root: [0; 32], content_len: 0, ..escrow() };
    match read_named(CHAIN, &c, &bare, &ready(&[1], BondCode::Active)) {
        Ok(Named::Voucher(v)) => v,
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_voucher_listing_reads_as_a_channel_without_escrow_fields() {
    let v = voucher();
    let b = buy();
    assert_eq!((v.chain_id, v.payee, v.listing_id), (CHAIN, b.payee, b.listing_id));
    assert_eq!((v.unit, v.unit_price, v.min_charge), (nonos_nox_config::UNIT_INSTALL, b.amount, 0));
}

#[test]
fn open_start_carries_the_deposit_expiry_and_reclaim_address() {
    let v = voucher();
    let s = v.open(v.unit_price * 3, NOW + 86_400, REFUND_TO, NOW).unwrap();
    let want = OpenStart {
        chain_id: CHAIN,
        payee: v.payee,
        listing_id: v.listing_id,
        unit: v.unit,
        amount: v.unit_price * 3,
        expiry: NOW + 86_400,
        reclaim_to: REFUND_TO,
    };
    assert_eq!(s, want);
    assert_eq!(parse_open(&s.encode()), Some(Ok(want)), "the meter reads what the wallet sends");
}

#[test]
fn a_deposit_under_one_use_a_past_expiry_or_no_reclaim_address_is_refused() {
    let v = voucher();
    assert!(v.open(v.unit_price - 1, NOW + 1, REFUND_TO, NOW).is_err());
    assert!(v.open(0, NOW + 1, REFUND_TO, NOW).is_err());
    assert!(v.open(v.unit_price, NOW, REFUND_TO, NOW).is_err(), "expiry is now");
    assert!(v.open(v.unit_price, NOW + 1, [0; 20], NOW).is_err());
    assert!(v.open(v.unit_price, NOW + 1, REFUND_TO, NOW).is_ok(), "exactly one use");
}
