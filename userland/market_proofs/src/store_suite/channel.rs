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
//! A voucher listing whose channel is open and funded installs on Enter, as
//! the meter's own FUNDED says (plan/21, plan/26); anything else posts.

use nonos_nox_config::SEPOLIA;
use nonos_payment_proto::AccountRow;

use super::held_fixture::{row, sold, NOW};
use super::meter_fixture::{meter, record};
use crate::meter::channel::CHARGE_MARGIN;
use crate::store::channel_held::{answer, route, Route};
use crate::store::price::VOUCHER;

fn over(rows: &[AccountRow]) -> Route {
    route(&sold(VOUCHER), answer(Ok(meter(rows, &record(3)))))
}

fn on(r: AccountRow) -> Route {
    over(&[r])
}

#[test]
fn an_open_funded_channel_for_this_listing_installs() {
    assert_eq!(on(row()), Route::Install);
    assert_eq!(on(AccountRow { cumulative: 7, ..row() }), Route::Install, "one install left");
    assert_eq!(over(&[AccountRow { listing_id: [8; 32], ..row() }, row()]), Route::Install);
    assert_eq!(on(AccountRow { expiry: NOW + CHARGE_MARGIN + 1, ..row() }), Route::Install);
}

#[test]
fn no_channel_another_listings_or_an_unreadable_meter_posts() {
    assert_eq!(over(&[]), Route::Post);
    assert_eq!(route(&sold(VOUCHER), None), Route::Post, "the meter did not answer");
    assert_eq!(route(&sold(VOUCHER), Some(false)), Route::Post);
    assert_eq!(on(AccountRow { listing_id: [8; 32], ..row() }), Route::Post);
    assert_eq!(on(AccountRow { chain_id: SEPOLIA + 1, ..row() }), Route::Post);
}

#[test]
fn a_channel_that_takes_no_more_installs_posts() {
    assert_eq!(on(AccountRow { cumulative: 8, ..row() }), Route::Post, "two left, the price three");
    let settled = AccountRow { settled: 8, cumulative: 0, ..row() };
    assert_eq!(on(settled), Route::Post, "settled counts");
    assert_eq!(on(AccountRow { cumulative: u128::MAX, deposit: u128::MAX, ..row() }), Route::Post);
    assert_eq!(on(AccountRow { frozen: true, ..row() }), Route::Post);
    let late = AccountRow { expiry: NOW + CHARGE_MARGIN, ..row() };
    assert_eq!(on(late), Route::Post, "within a day");
}

#[test]
fn the_records_price_decides_not_the_stores() {
    let l = sold(VOUCHER);
    assert_eq!(route(&l, answer(Ok(meter(&[row()], &record(0))))), Route::Post, "PAY refuses 0");
    assert_eq!(route(&l, answer(Ok(meter(&[row()], &record(7))))), Route::Post, "6 left, 7 a go");
    let mut cheap = sold(VOUCHER);
    if let Some(p) = cheap.known.price.as_mut() {
        p.unit_price = 1;
    }
    let dear = AccountRow { cumulative: 8, ..row() };
    assert_eq!(route(&cheap, answer(Ok(meter(&[dear], &record(3))))), Route::Post, "2 left at 3");
}
