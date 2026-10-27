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

//! What mmap refuses of a descriptor mapped shared, and in which order.

use crate::linux::abi::errno::{EACCES, ENODEV};
use crate::linux::file::shared_maps::asks::refused;

#[test]
fn only_a_stored_file_is_mapped_shared() {
    assert_eq!(refused(false, true, true, false, false), Some(ENODEV));
    assert_eq!(refused(false, false, false, true, true), Some(ENODEV), "ENODEV comes first");
}

#[test]
fn the_descriptor_must_read_and_write_only_if_it_was_opened_to() {
    assert_eq!(refused(true, false, true, false, false), Some(EACCES), "not open to read");
    assert_eq!(refused(true, true, false, true, false), Some(EACCES), "write, opened read-only");
    assert_eq!(refused(true, true, false, false, false), None, "read, opened read-only");
    assert_eq!(refused(true, true, true, true, false), None, "write, opened to write");
}

#[test]
fn no_shared_file_map_is_ever_executable() {
    assert_eq!(refused(true, true, true, false, true), Some(EACCES));
    assert_eq!(refused(true, true, false, false, true), Some(EACCES));
}
