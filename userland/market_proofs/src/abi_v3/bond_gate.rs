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

use nonos_marketplace_abi::{bond_verdict, BondReading, BondVerdict, MarketplaceEntry as E};
use nonos_marketplace_abi::{SettleMode, BOND_MAX_AGE};

use super::fixtures::sold;

/// The bond gate (B6): readings against the entry that names the bond.
const NOW: u64 = 1_800_000_000;
const MIN: u128 = 1_000;

fn entry(mode: SettleMode, price: u128) -> E {
    let mut e = sold().entries[0].clone();
    let c = e.commerce.as_mut().unwrap();
    (c.settle_mode, c.unit_price) = (mode, price);
    e
}

fn reading(e: &E, amount: u128) -> BondReading {
    BondReading {
        payee: e.commerce.as_ref().unwrap().payee,
        publisher_key: e.publisher_pubkey,
        amount,
        unlock_at: 0,
        slashed: 0,
        active: true,
        read_at: NOW - 10,
    }
}

#[test]
fn a_sold_listing_needs_its_own_active_bond() {
    let e = entry(SettleMode::Voucher, 5_000);
    let good = reading(&e, MIN);
    let v = |r: Option<&BondReading>| bond_verdict(&e, r, MIN, NOW);
    assert_eq!(v(Some(&good)), BondVerdict::Active);
    assert_eq!(v(None), BondVerdict::Unread);
    let at = |t: u64| BondReading { read_at: t, ..good };
    assert_eq!(v(Some(&at(NOW - BOND_MAX_AGE))), BondVerdict::Active);
    assert_eq!(v(Some(&at(NOW - BOND_MAX_AGE - 1))), BondVerdict::Unread);
    assert_eq!(v(Some(&at(NOW + 1))), BondVerdict::Unread);
    assert_eq!(v(Some(&BondReading { payee: [0xEE; 20], ..good })), BondVerdict::NotThisListing);
    assert_eq!(
        v(Some(&BondReading { publisher_key: [0xEE; 32], ..good })),
        BondVerdict::NotThisListing
    );
    assert_eq!(v(Some(&BondReading { active: false, ..good })), BondVerdict::Inactive);
    assert_eq!(v(Some(&BondReading { unlock_at: NOW, ..good })), BondVerdict::Inactive);
    assert_eq!(v(Some(&BondReading { amount: MIN - 1, ..good })), BondVerdict::BelowMinimum);
}

#[test]
fn escrow_is_priced_within_the_bond_and_free_needs_none() {
    let e = entry(SettleMode::Escrow, 5_000);
    assert_eq!(bond_verdict(&e, Some(&reading(&e, 5_000)), MIN, NOW), BondVerdict::Active);
    assert_eq!(bond_verdict(&e, Some(&reading(&e, 4_999)), MIN, NOW), BondVerdict::PriceAboveBond);
    let free = entry(SettleMode::Free, 0);
    assert_eq!(bond_verdict(&free, None, MIN, NOW), BondVerdict::NotNeeded);
    let v2 = E { commerce: None, ..free };
    assert_eq!(bond_verdict(&v2, None, MIN, NOW), BondVerdict::NotNeeded);
}
