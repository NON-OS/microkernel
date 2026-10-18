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

//! A wait's budget on a clock that can fail: what is left, by the clock,
//! and nothing once a read fails or the time is spent.

use crate::clock::Budget;

#[test]
fn the_budget_counts_down_by_the_clock_and_never_wraps() {
    let b = Budget::begin(Some(1_000), 5_000);
    assert_eq!(b.left(Some(1_000)), 5_000);
    assert_eq!(b.left(Some(5_999)), 1);
    assert_eq!(b.left(Some(6_000)), 0);
    assert_eq!(b.left(Some(u64::MAX)), 0);
    assert_eq!(b.left(Some(0)), 5_000, "a clock gone backwards is no time passed");
    assert_eq!(Budget::begin(Some(u64::MAX), u64::MAX).left(Some(u64::MAX)), u64::MAX);
}

#[test]
fn a_clock_that_cannot_be_read_spends_the_budget() {
    assert_eq!(Budget::begin(None, 5_000).left(Some(0)), 0);
    assert_eq!(Budget::begin(Some(0), 5_000).left(None), 0);
    assert_eq!(Budget::begin(None, u64::MAX).left(None), 0);
}
