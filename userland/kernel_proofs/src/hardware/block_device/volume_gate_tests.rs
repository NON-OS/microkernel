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

use super::volume_rule::admits;

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
