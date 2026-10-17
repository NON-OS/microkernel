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
//! The owner's budget, global and for the listing, sends the store to the
//! wallet where it refuses PAY of one install unit, and nowhere else.

use super::held_fixture::{row, KEY, NOW};
use super::meter_state::{budget, channel, state};
use super::pay_once::same;
use crate::meter::budget::{Budget, ListingCap};
use crate::meter::state::MeterState;

fn spends(b: Budget) -> MeterState {
    MeterState { budget: b, ..state(&[channel(&row())]) }
}

fn capped(cap: u128, listing_id: [u8; 32]) -> MeterState {
    spends(Budget { listings: vec![ListingCap { listing_id, cap, spent: 0 }], ..budget() })
}

#[test]
fn the_budget_decides_the_store_as_it_decides_pay() {
    same(&[
        ("global cap short", spends(Budget { cap: 2, ..budget() }), false),
        ("global cap exact", spends(Budget { cap: 3, ..budget() }), true),
        ("global spent", spends(Budget { spent: 98, ..budget() }), false),
        ("budget expired", spends(Budget { expiry: NOW, ..budget() }), false),
        ("no budget", spends(Budget::default()), false),
        ("listing cap short", capped(2, KEY), false),
        ("listing cap exact", capped(3, KEY), true),
        ("another listing's cap", capped(1, [8; 32]), true),
    ]);
}
