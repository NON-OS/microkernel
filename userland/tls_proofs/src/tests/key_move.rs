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

//! The secrets live in one block a `TrafficKeys` points to, filled in place:
//! moving the keys copies a pointer and the suite, never a secret.

use crate::traffic_keys::{Secrets, TrafficKeys};

fn keys() -> TrafficKeys {
    TrafficKeys::build(0x1303, |s| {
        s.client_key = [4; 32];
        s.server_iv = [7; 12];
        true
    })
    .expect("filled")
}

#[test]
fn a_move_copies_a_pointer_and_the_suite_never_a_secret() {
    assert!(core::mem::size_of::<TrafficKeys>() <= 2 * core::mem::size_of::<usize>());
    let k = keys();
    let block = &*k as *const Secrets;
    let moved = [k];
    assert_eq!(&*moved[0] as *const Secrets, block, "the secrets did not move");
    let copy = moved[0].clone();
    assert_ne!(&*copy as *const Secrets, block, "a clone is a block of its own");
    assert_eq!(copy.client_key, [4; 32]);
}

#[test]
fn keys_are_filled_in_place_and_a_failed_fill_yields_none() {
    assert_eq!(keys().server_iv, [7; 12]);
    assert!(TrafficKeys::build(0x1301, |s| {
        s.client_key = [9; 32];
        false
    })
    .is_none());
}
