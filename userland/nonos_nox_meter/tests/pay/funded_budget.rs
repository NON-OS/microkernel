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

//! The owner's budget, global and for the listing, refuses FUNDED where
//! it refuses PAY of one unit, and nowhere else.

use nonos_nox_meter::budget::{Budget, ListingCap};
use nonos_nox_meter::state::MeterState;

use super::funded_fixture::{budget, channel, state, NOW};
use super::funded_pay::same;

fn spends(b: Budget) -> MeterState {
    MeterState { budget: b, ..state(&[channel()]) }
}

fn capped(cap: u128, listing_id: [u8; 32]) -> MeterState {
    spends(Budget { listings: vec![ListingCap { listing_id, cap, spent: 0 }], ..budget() })
}

#[test]
fn the_budget_decides_funded_as_it_decides_pay() {
    same(&[
        ("global cap short", spends(Budget { cap: 6, ..budget() }), false),
        ("global cap exact", spends(Budget { cap: 7, ..budget() }), true),
        ("global spent", spends(Budget { spent: 94, ..budget() }), false),
        ("budget expired", spends(Budget { expiry: NOW, ..budget() }), false),
        ("no budget", spends(Budget::default()), false),
        ("listing cap short", capped(6, [1; 32]), false),
        ("listing cap exact", capped(7, [1; 32]), true),
        ("another listing's cap", capped(1, [8; 32]), true),
    ]);
}
