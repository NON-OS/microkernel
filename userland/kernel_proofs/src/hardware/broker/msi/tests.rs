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

//! The MSI side table (src/hardware/broker/pci_index/msi*): bind and teardown
//! read the same entry for a handle, and a departed function's entry goes with
//! it, so a later handle never reads a stale one.

use super::set::set;
use super::table::of;

#[test]
fn every_read_of_a_handle_answers_its_own_entry() {
    let mut t = Vec::new();
    set(&mut t, 3, Some('a'));
    set(&mut t, 7, Some('b'));
    // Bind, then teardown, then bind again: one entry, the same each time.
    assert_eq!([of(&t, 3), of(&t, 3), of(&t, 3)], [Some('a'); 3]);
    assert_eq!(of(&t, 7), Some('b'));
    assert_eq!(of(&t, 4), None);
}

#[test]
fn a_set_replaces_and_never_leaves_two() {
    let mut t = Vec::new();
    set(&mut t, 3, Some(1u8));
    set(&mut t, 3, Some(2u8));
    assert_eq!((of(&t, 3), t.len()), (Some(2), 1));
}

#[test]
fn a_departed_function_leaves_no_entry_for_any_later_handle() {
    let mut t = Vec::new();
    set(&mut t, 3, Some(9u8));
    set::<u8>(&mut t, 3, None);
    assert!(t.is_empty());
    // A later arrival, under its own new id or read back under the old one.
    assert_eq!((of(&t, 3), of(&t, 4)), (None, None));
    set(&mut t, 4, Some(5u8));
    assert_eq!((of(&t, 3), of(&t, 4)), (None, Some(5)));
}
