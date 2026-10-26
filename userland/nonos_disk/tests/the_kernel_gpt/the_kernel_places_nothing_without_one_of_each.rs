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

//! No places from a table missing a NONOS partition, doubling one, or with a
//! plan too small for the key header behind it; a missing data partition is
//! only absent.

use crate::gpt_damage::{put, seal, table, T};
use crate::kernel_gpt::Which;
use crate::kernel_gpt::{entries, header, places};

#[test]
fn a_nonos_partition_missing_doubled_or_too_small_places_nothing() {
    let (mut h, a0) = table();
    let read = |h: &mut Vec<u8>, a: &[u8]| {
        seal(h, a);
        let hd = header::parse_as(h, T, Which::Primary).unwrap();
        places(&entries(&hd, a).unwrap())
    };
    assert!(read(&mut h, &a0).is_some());
    let hd = header::parse_as(&h, T, Which::Primary).unwrap();
    let end = entries(&hd, &a0).unwrap().iter().map(|e| e.last).max().unwrap();
    assert!(end + 8 <= hd.last_usable, "room for a second store");
    let mut a = a0.clone();
    a[128..256].fill(0);
    assert_eq!(read(&mut h, &a), None, "no plan");
    // The second store sits in sectors no other partition uses: a copy in
    // place would overlap the first, and the table would name nothing.
    let mut a = a0.clone();
    a.copy_within(0..128, 4 * 128);
    put(&mut a, 4 * 128 + 32, end + 1, 8);
    put(&mut a, 4 * 128 + 40, end + 8, 8);
    assert_eq!(read(&mut h, &a), None, "two stores");
    let mut a = a0.clone();
    let plan_first = u64::from_le_bytes(a[128 + 32..128 + 40].try_into().unwrap());
    put(&mut a, 128 + 40, plan_first, 8);
    assert_eq!(read(&mut h, &a), None, "a plan with no room for the key header");
    let mut a = a0.clone();
    a[256..384].fill(0);
    assert_eq!(read(&mut h, &a).map(|p| p.data), Some(None), "no data partition");
}
