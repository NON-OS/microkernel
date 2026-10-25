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
//! FUNDED's channel half: a channel for the record's listing and chain, not
//! frozen, live past the margin, with room for one unit at its price.

use nonos_nox_meter::channel::{Channel, CHARGE_MARGIN};
use nonos_nox_meter::pay::funded;

use super::fixture::NOX;
use super::funded_fixture::{channel, record, state, NOW};

fn on(channels: &[Channel]) -> bool {
    funded(&state(channels), &record(), 1, &NOX, NOW)
}

#[test]
fn a_row_for_the_records_listing_with_room_for_one_unit_is_funded() {
    assert!(on(&[channel()]));
    assert!(on(&[Channel { cumulative: 13, ..channel() }]), "exactly one install left");
    let other = Channel { listing_id: [8; 32], ..channel() };
    assert!(on(&[other, channel()]), "any row of the listing's");
    assert!(on(&[Channel { expiry: NOW + CHARGE_MARGIN + 1, ..channel() }]));
}

#[test]
fn no_row_or_another_listings_or_chains_is_not_funded() {
    assert!(!on(&[]));
    assert!(!on(&[Channel { listing_id: [8; 32], ..channel() }]));
    assert!(!on(&[Channel { chain_id: 2, ..channel() }]));
    assert!(!funded(&state(&[Channel { chain_id: 2, ..channel() }]), &record(), 2, &NOX, NOW));
}

#[test]
fn a_row_that_takes_no_more_units_is_not_funded() {
    assert!(!on(&[Channel { cumulative: 14, ..channel() }]), "six left, the price seven");
    assert!(!on(&[Channel { settled: 14, cumulative: 0, ..channel() }]), "settled counts");
    let full = Channel { cumulative: u128::MAX, deposit: u128::MAX, ..channel() };
    assert!(!on(&[full]), "checked, never wrapped");
    assert!(!on(&[Channel { frozen: true, ..channel() }]));
    assert!(!on(&[Channel { expiry: NOW + CHARGE_MARGIN, ..channel() }]), "within a day");
    assert!(!on(&[Channel { expiry: 0, ..channel() }]));
}
