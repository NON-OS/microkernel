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

//! Entries the kernel must not use (UEFI 2.10 5.3.3, as recalled,
//! unverified): an array whose CRC fails names nothing; an entry outside
//! the usable range, or ending before it starts, is skipped; partitions
//! that overlap make the table name nothing.

use crate::gpt_damage::{put, seal, table, T};
use crate::kernel_gpt::Which;
use crate::kernel_gpt::{entries, header, places};

#[test]
fn a_damaged_array_or_an_entry_out_of_range_names_nothing_wrong() {
    let (mut h, mut a) = table();
    let hd = header::parse_as(&h, T, Which::Primary).unwrap();
    a[5] ^= 1;
    assert!(entries(&hd, &a).is_none(), "the array CRC");
    a[5] ^= 1;
    // The store's entry (0) moved below the first usable sector, and the
    // plan's (1) turned inside out: both are dropped, so no places.
    put(&mut a, 32, 33, 8);
    put(&mut a, 128 + 32, 300_000, 8);
    seal(&mut h, &a);
    let hd = header::parse_as(&h, T, Which::Primary).unwrap();
    let e = entries(&hd, &a).unwrap();
    assert_eq!(e.len(), 2, "data and ESP remain");
    assert_eq!(places(&e), None);
}

#[test]
fn partitions_that_overlap_name_nothing() {
    let (mut h, mut a) = table();
    let hd = header::parse_as(&h, T, Which::Primary).unwrap();
    let e = entries(&hd, &a).unwrap();
    // The plan's entry (1) moved to start on the store's (0) last sector.
    put(&mut a, 128 + 32, e[0].last, 8);
    seal(&mut h, &a);
    let hd = header::parse_as(&h, T, Which::Primary).unwrap();
    assert!(entries(&hd, &a).is_none(), "one sector shared");
    put(&mut a, 128 + 32, e[0].last + 1, 8);
    seal(&mut h, &a);
    let hd = header::parse_as(&h, T, Which::Primary).unwrap();
    assert!(entries(&hd, &a).is_some(), "touching is not sharing");
}
