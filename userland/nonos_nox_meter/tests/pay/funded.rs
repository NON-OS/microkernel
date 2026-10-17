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
//! FUNDED's channel half: a row for the record's listing and chain, not
//! frozen, live past the margin, with room for one unit at its price.

use nonos_nox_meter::channel::CHARGE_MARGIN;
use nonos_nox_meter::pay::funded;
use nonos_payment_proto::AccountRow;

use super::fixture::NOX;
use super::funded_fixture::{record, row, NOW};

fn on(rows: &[AccountRow]) -> bool {
    funded(rows.iter().copied(), &record(), 1, &NOX, NOW)
}

#[test]
fn a_row_for_the_records_listing_with_room_for_one_unit_is_funded() {
    assert!(on(&[row()]));
    assert!(on(&[AccountRow { cumulative: 13, ..row() }]), "exactly one install left");
    let other = AccountRow { listing_id: [8; 32], ..row() };
    assert!(on(&[other, row()]), "any row of the listing's");
    assert!(on(&[AccountRow { expiry: NOW + CHARGE_MARGIN + 1, ..row() }]));
}

#[test]
fn no_row_or_another_listings_or_chains_is_not_funded() {
    assert!(!on(&[]));
    assert!(!on(&[AccountRow { listing_id: [8; 32], ..row() }]));
    assert!(!on(&[AccountRow { chain_id: 2, ..row() }]));
    assert!(!funded([AccountRow { chain_id: 2, ..row() }], &record(), 2, &NOX, NOW));
}

#[test]
fn a_row_that_takes_no_more_units_is_not_funded() {
    assert!(!on(&[AccountRow { cumulative: 14, ..row() }]), "six left, the price seven");
    assert!(!on(&[AccountRow { settled: 14, cumulative: 0, ..row() }]), "settled counts");
    let full = AccountRow { cumulative: u128::MAX, deposit: u128::MAX, ..row() };
    assert!(!on(&[full]), "checked, never wrapped");
    assert!(!on(&[AccountRow { frozen: true, ..row() }]));
    assert!(!on(&[AccountRow { expiry: NOW + CHARGE_MARGIN, ..row() }]), "within a day");
    assert!(!on(&[AccountRow { expiry: 0, ..row() }]));
}
