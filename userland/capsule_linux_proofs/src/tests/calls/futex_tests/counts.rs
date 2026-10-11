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

//! Wake and requeue counts, read as the ints they are.

use crate::linux::abi::errno::EINVAL;
use crate::linux::call::futex_op::{requeue_counts, wake_count};

#[test]
fn a_wake_count_is_an_int_and_wakes_at_least_one() {
    assert_eq!(wake_count(0), 1, "Linux wakes one before it compares");
    assert_eq!(wake_count(1), 1);
    assert_eq!(wake_count(5), 5);
    assert_eq!(wake_count(i32::MAX as u64), i32::MAX as u64);
    assert_eq!(wake_count(u32::MAX as u64), 1, "-1 as an int");
    assert_eq!(wake_count(u64::MAX), 1, "-1 sign-extended by the caller");
    assert_eq!(wake_count((1 << 32) | 3), 3, "the upper half is not the int's");
}

#[test]
fn requeue_counts_are_ints_and_a_negative_one_is_refused() {
    assert_eq!(requeue_counts(1, 2), Ok((1, 2)));
    assert_eq!(requeue_counts(0, 0), Ok((0, 0)));
    assert_eq!(requeue_counts(u64::MAX, 0), Err(EINVAL));
    assert_eq!(requeue_counts(0, 0x8000_0000), Err(EINVAL));
    assert_eq!(requeue_counts(i32::MAX as u64, (1 << 32) | 1), Ok((i32::MAX as u64, 1)));
}
