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

//! How a stream reads an END reason, a CONNECTED body and an id.

extern crate alloc;

use alloc::vec;

use crate::stream::connected::is_valid;
use crate::stream::end::{is_clean, needs_another_exit, reason};
use crate::stream::ids::next;

#[test]
fn reasons_are_read_and_classified() {
    assert_eq!(reason(&[6]), 6);
    assert!(is_clean(reason(&[6])));
    assert!(!is_clean(reason(&[4])));
    // An empty body is a relay declining to say why, reported as MISC.
    assert_eq!(reason(&[]), 1);
}

#[test]
fn only_a_permanent_refusal_forces_another_exit() {
    assert!(needs_another_exit(4));
    assert!(needs_another_exit(8));
    assert!(needs_another_exit(9));
    assert!(!needs_another_exit(7));
    assert!(!needs_another_exit(11));
    assert!(!needs_another_exit(6));
}

#[test]
fn a_connected_body_is_checked_for_shape() {
    assert!(is_valid(&[]));
    assert!(is_valid(&[1, 2, 3, 4, 0, 0, 1, 0]));
    assert!(!is_valid(&[1, 2, 3]));
    let mut six = vec![0u8; 25];
    six[4] = 6;
    assert!(is_valid(&six), "an IPv6 answer is well formed");
    six[4] = 9;
    assert!(!is_valid(&six), "an unknown address type is not");
}

#[test]
fn stream_ids_never_reach_zero() {
    assert_eq!(next(1), 2);
    assert_eq!(next(u16::MAX), 1);
    assert_ne!(next(u16::MAX), 0);
}
