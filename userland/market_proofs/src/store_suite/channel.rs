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

//! A voucher listing whose channel is open and funded installs on Enter
//! (plan/21 ACCOUNT, plan/26 the day before expiry); anything else posts.

use nonos_nox_config::SEPOLIA;
use nonos_payment_proto::AccountRow;

use super::held_fixture::{row, sold, NOW};
use crate::store::channel_held::{route, Route, CHARGE_MARGIN};
use crate::store::price::VOUCHER;

fn on(r: AccountRow) -> Route {
    route(&sold(VOUCHER), Some(&[r]), NOW)
}

#[test]
fn an_open_funded_channel_for_this_listing_installs() {
    assert_eq!(on(row()), Route::Install);
    assert_eq!(
        on(AccountRow { cumulative: 7, ..row() }),
        Route::Install,
        "exactly one install left"
    );
    let other = AccountRow { listing_id: [8; 32], ..row() };
    assert_eq!(route(&sold(VOUCHER), Some(&[other, row()]), NOW), Route::Install);
}

#[test]
fn no_channel_another_listings_or_an_unreadable_meter_posts() {
    assert_eq!(route(&sold(VOUCHER), Some(&[]), NOW), Route::Post);
    assert_eq!(route(&sold(VOUCHER), None, NOW), Route::Post, "the meter did not answer");
    assert_eq!(on(AccountRow { listing_id: [8; 32], ..row() }), Route::Post);
    assert_eq!(on(AccountRow { chain_id: SEPOLIA + 1, ..row() }), Route::Post);
}

#[test]
fn a_channel_that_takes_no_more_installs_posts() {
    assert_eq!(on(AccountRow { cumulative: 8, ..row() }), Route::Post, "two left, the price three");
    assert_eq!(
        on(AccountRow { settled: 8, cumulative: 0, ..row() }),
        Route::Post,
        "settled counts"
    );
    assert_eq!(on(AccountRow { cumulative: u128::MAX, deposit: u128::MAX, ..row() }), Route::Post);
    assert_eq!(on(AccountRow { frozen: true, ..row() }), Route::Post);
    assert_eq!(
        on(AccountRow { expiry: NOW + CHARGE_MARGIN, ..row() }),
        Route::Post,
        "within a day"
    );
    assert_eq!(on(AccountRow { expiry: NOW + CHARGE_MARGIN + 1, ..row() }), Route::Install);
    let mut free_unit = sold(VOUCHER);
    if let Some(p) = free_unit.known.price.as_mut() {
        p.unit_price = 0;
    }
    assert_eq!(route(&free_unit, Some(&[row()]), NOW), Route::Post, "PAY refuses a zero charge");
}
