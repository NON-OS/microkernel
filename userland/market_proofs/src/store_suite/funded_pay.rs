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
//! FUNDED says one, and the store installs, exactly when PAY of one install
//! unit would charge, for each way a channel can stand.

use nonos_nox_config::SEPOLIA;
use nonos_payment_proto::AccountRow;

use super::held_fixture::{row, NOW};
use super::meter_state::{channel, state};
use super::pay_once::same;
use crate::meter::channel::{Channel, CHARGE_MARGIN};

#[test]
fn the_store_installs_exactly_when_pay_of_one_unit_charges() {
    let on = |r: AccountRow| state(&[channel(&r)]);
    let with = |c: Channel| state(&[c]);
    let held = channel(&row());
    same(&[
        ("funded", on(row()), true),
        ("one install left", on(AccountRow { cumulative: 7, ..row() }), true),
        ("payee mismatch", with(Channel { payee: [0xcc; 20], ..held }), false),
        ("unit mismatch", with(Channel { unit: [4; 32], ..held }), false),
        ("no row", state(&[]), false),
        ("frozen", on(AccountRow { frozen: true, ..row() }), false),
        ("within the margin", on(AccountRow { expiry: NOW + CHARGE_MARGIN, ..row() }), false),
        ("past the margin", on(AccountRow { expiry: NOW + CHARGE_MARGIN + 1, ..row() }), true),
        ("underfunded", on(AccountRow { cumulative: 8, ..row() }), false),
        ("overflow", on(AccountRow { cumulative: u128::MAX, deposit: u128::MAX, ..row() }), false),
        ("another listing", on(AccountRow { listing_id: [8; 32], ..row() }), false),
        ("another chain", on(AccountRow { chain_id: SEPOLIA + 1, ..row() }), false),
    ]);
}
