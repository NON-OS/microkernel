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

//! The block clients' gate, pure: a caller with neither Driver nor
//! StoreWrite is refused unless the kernel is doing the data volume's own
//! I/O for it; a driver and the store's holder pass as before.

use super::volume_rule::{admits, free_slot, inside};

#[test]
fn a_caller_with_neither_cap_is_refused_outside_the_volume() {
    assert!(!admits(false, false, false));
}

#[test]
fn the_volume_s_own_io_passes_for_any_caller() {
    for driver in [false, true] {
        for storage in [false, true] {
            assert!(admits(true, driver, storage));
        }
    }
}

#[test]
fn a_driver_or_the_store_s_holder_passes_as_before() {
    assert!(admits(false, true, false));
    assert!(admits(false, false, true));
    assert!(admits(false, true, true));
}

/// A scope is the thread's own: a sibling of the same process, another tid,
/// is not inside while the first thread's volume call runs.
#[test]
fn a_sibling_thread_is_held_to_its_own_caps() {
    let mut slots = [0u32; 16];
    slots[3] = 41;
    assert!(inside(&slots, 41));
    assert!(!inside(&slots, 42), "a sibling thread");
    assert!(!admits(inside(&slots, 42), false, false));
    assert!(!inside(&[0; 16], 0), "no thread is the empty slot");
}

/// Sixteen scopes at once; the seventeenth finds no slot and is refused.
#[test]
fn a_seventeenth_scope_finds_no_slot() {
    let mut slots = [0u32; 16];
    for tid in 1..=16 {
        let i = free_slot(&slots).expect("a slot for each of sixteen");
        slots[i] = tid;
    }
    assert_eq!(free_slot(&slots), None);
    slots[7] = 0;
    assert_eq!(free_slot(&slots), Some(7), "a released slot is taken again");
}
