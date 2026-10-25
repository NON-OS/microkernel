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

use super::fixture::{LISTING, NOW};
use nonos_nox_meter::budget::{Budget, ListingCap};
use nonos_nox_meter::MeterError;

#[test]
fn the_default_budget_allows_nothing() {
    assert_eq!(Budget::default().spending(&LISTING, 1, NOW), Err(MeterError::Expired));
}

#[test]
fn the_global_cap_and_a_listing_cap_both_hold() {
    let cap = ListingCap { listing_id: LISTING, cap: 100, spent: 0 };
    let b = Budget { cap: 150, spent: 0, expiry: NOW + 1, listings: vec![cap] };
    let b = b.spending(&LISTING, 100, NOW).unwrap();
    assert_eq!(b.spending(&LISTING, 1, NOW), Err(MeterError::OverBudget));
    let other = [7u8; 32];
    let b = b.spending(&other, 50, NOW).unwrap();
    assert_eq!(b.spending(&other, 1, NOW), Err(MeterError::OverBudget));
    assert_eq!(b.spending(&other, 0, NOW + 1), Err(MeterError::Expired));
}

#[test]
fn absorbed_spend_can_overdraw_but_is_never_hidden() {
    let mut b = Budget { cap: 10, spent: 8, expiry: NOW + 1, listings: vec![] };
    b.absorb(&LISTING, 5);
    assert_eq!(b.spent, 13);
    assert_eq!(b.spending(&LISTING, 0, NOW), Err(MeterError::OverBudget));
}

#[test]
fn a_fresh_budget_starts_unspent_and_refuses_a_listing_twice() {
    let b = Budget::fresh(500, NOW + 10, &[(LISTING, 40)]).unwrap();
    assert_eq!((b.spent, b.listings[0].spent, b.left(NOW), b.left(NOW + 10)), (0, 0, 500, 0));
    let twice = [(LISTING, 1), (LISTING, 2)];
    assert_eq!(Budget::fresh(1, NOW, &twice), Err(MeterError::Duplicate));
    let many: Vec<_> = (0..33u8).map(|i| ([i; 32], 1)).collect();
    assert_eq!(Budget::fresh(1, NOW, &many), Err(MeterError::Full));
}
