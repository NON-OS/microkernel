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

//! FUTEX_WAKE_OP is served, and its operation read as Linux reads it.

use crate::linux::abi::errno::ENOSYS;
use crate::linux::call::futex_op::{decode_op, is_wake_op};

/// FUTEX_OP(op, oparg, cmp, cmparg), as linux/futex.h builds it.
fn encode(op: u32, oparg: u32, cmp: u32, cmparg: u32) -> u64 {
    u64::from(((op & 0xf) << 28) | ((cmp & 0xf) << 24) | ((oparg & 0xfff) << 12) | (cmparg & 0xfff))
}

#[test]
fn wake_op_is_named_private_or_not_but_not_with_the_realtime_clock() {
    assert!(is_wake_op(5));
    assert!(is_wake_op(5 | 128));
    assert!(!is_wake_op(5 | 256));
    assert!(!is_wake_op(1));
}

#[test]
fn qt_semaphore_release_wakes_the_second_word_and_leaves_it_alone() {
    /* QSemaphore::release: OR 0, compared not equal to 0. */
    let op = decode_op(encode(2, 0, 1, 0)).unwrap();
    assert_eq!(op.apply(0x8000_0001), 0x8000_0001);
    assert!(op.holds(0x8000_0001));
    assert!(!op.holds(0));
}

#[test]
fn each_operation_and_comparison_is_linux_own() {
    assert_eq!(decode_op(encode(0, 7, 0, 0)).unwrap().apply(3), 7);
    assert_eq!(decode_op(encode(1, 0xfff, 0, 0)).unwrap().apply(3), 2);
    assert_eq!(decode_op(encode(3, 1, 0, 0)).unwrap().apply(3), 2);
    assert_eq!(decode_op(encode(4, 5, 0, 0)).unwrap().apply(3), 6);
    /* The shift flag makes the argument a bit: OR 1 << 4. */
    assert_eq!(decode_op(encode(8 | 2, 4, 0, 0)).unwrap().apply(1), 17);
    /* The comparison is of ints, and its argument is signed. */
    let less = decode_op(encode(0, 0, 2, 0)).unwrap();
    assert!(less.holds(u32::MAX));
    let eq_minus_one = decode_op(encode(0, 0, 0, 0xfff)).unwrap();
    assert!(eq_minus_one.holds(u32::MAX));
    for (cmp, old, want) in [(3, 5, false), (4, 5, true), (5, 0, true)] {
        assert_eq!(decode_op(encode(0, 0, cmp, 0)).unwrap().holds(old), want, "cmp {cmp}");
    }
}

#[test]
fn an_unknown_operation_or_comparison_is_enosys() {
    assert_eq!(decode_op(encode(5, 0, 0, 0)), Err(ENOSYS));
    assert_eq!(decode_op(encode(0, 0, 6, 0)), Err(ENOSYS));
}
