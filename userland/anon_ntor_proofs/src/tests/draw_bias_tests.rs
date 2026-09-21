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

//! That the draw does not lean on the front of a list, and its scaling.

extern crate alloc;

use alloc::vec;

use crate::draw::{pick, scale};

/*
 * A drawer that favoured the front of the list would still pass a range check,
 * and would quietly pin every client in the fleet to the same guard, because a
 * consensus arrives in a stable order. The live consensus carries 4092 Guard
 * flagged relays, so that is the width this is checked at.
 */
#[test]
fn a_flat_list_does_not_favour_the_front() {
    let candidates = 4_092usize;
    let weights = vec![1u64; candidates];
    let mut first = 0u64;
    for roll in 0..candidates as u64 {
        if pick(&weights, roll) == Some(0) {
            first += 1;
        }
    }
    assert_eq!(first, 1, "index 0 was chosen {first} times out of {candidates}");
}

#[test]
fn scaling_is_by_ten_thousandths_and_does_not_overflow() {
    /*
     * A `w Bandwidth=` line on the live consensus reaches 20000, and the
     * position weights are in units of ten thousand.
     */
    assert_eq!(scale(20_000, 10_000), 20_000);
    assert_eq!(scale(20_000, 1_893), 3_786);
    assert_eq!(scale(0, 10_000), 0);
    assert_eq!(scale(u32::MAX, 10_000), u32::MAX as u64);
}
