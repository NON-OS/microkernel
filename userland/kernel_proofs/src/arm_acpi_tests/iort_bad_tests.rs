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

//! IORT mappings that name nothing or overrun their node.

use super::build::*;
use super::iort_build::*;
use crate::arm_acpi::iort::iort;
use crate::arm_acpi::sdt::TableError;

#[test]
fn mappings_that_name_nothing_or_overrun() {
    let mut v = nodes();
    v[2] = node(2, 1, 2, &[0; 20], &[mapping((0, 0xFF, 0, 999, 0))]);
    assert_eq!(iort(&iort_of(&v)), Err(TableError::Reference(RC as usize)));
    let mut v = nodes();
    v[1][8..12].copy_from_slice(&u32le(2));
    assert_eq!(
        iort(&iort_of(&v)),
        Err(TableError::Structure(SMMU as usize)),
        "two mappings in room for one"
    );
    let mut v = nodes();
    v[1][12..16].copy_from_slice(&u32le(8));
    assert_eq!(
        iort(&iort_of(&v)),
        Err(TableError::Structure(SMMU as usize)),
        "mappings over the header"
    );
    let mut v = nodes();
    v[0][16..20].copy_from_slice(&u32le(1000));
    assert_eq!(
        iort(&iort_of(&v)),
        Err(TableError::Structure(ITS as usize)),
        "ITS IDs past the node"
    );
    let mut t = iort_of(&nodes());
    t[36..40].copy_from_slice(&u32le(5));
    seal(&mut t);
    assert!(iort(&t).is_err(), "a count past the nodes there are");
    let mut v = nodes();
    v[3][1..3].copy_from_slice(&u16le(10));
    assert!(iort(&iort_of(&v)).is_err(), "a node shorter than its header");
}
