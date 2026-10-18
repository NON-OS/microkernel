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

//! poll's and select's counts, and select's sets.

use crate::linux::abi::errno::EINVAL;
use crate::linux::net::ready_sets::{
    first_closed, is_set, narrow, poll_count, select_count, set_bytes,
};

pub(super) const MOST: u64 = 256;
pub(super) const IN: u32 = 0x001;
pub(super) const OUT: u32 = 0x004;
pub(super) const HUP: u32 = 0x010;

#[test]
fn poll_counts_an_unsigned_int_no_larger_than_rlimit_nofile() {
    assert_eq!(poll_count(0, MOST), Ok(0));
    assert_eq!(poll_count(MOST, MOST), Ok(MOST));
    assert_eq!(poll_count(MOST + 1, MOST), Err(EINVAL));
    assert_eq!(poll_count(u32::MAX as u64, MOST), Err(EINVAL));
    assert_eq!(poll_count(u64::MAX, MOST), Err(EINVAL));
    assert_eq!(poll_count((1 << 32) | 3, MOST), Ok(3), "the upper half is not the count's");
}

#[test]
fn select_counts_an_int_refuses_a_negative_one_and_looks_no_further_than_the_table() {
    assert_eq!(select_count(u64::MAX, MOST), Err(EINVAL));
    assert_eq!(select_count(0x8000_0000, MOST), Err(EINVAL));
    assert_eq!(select_count(1024, MOST), Ok(MOST), "a bigger set is fine, as on Linux");
    assert_eq!(select_count(5, MOST), Ok(5));
    assert_eq!(select_count((1 << 32) | 5, MOST), Ok(5));
    assert_eq!([0, 1, 64, 65, 256].map(set_bytes), [0, 8, 8, 16, 32]);
}

#[test]
fn select_names_the_first_closed_descriptor_in_any_set() {
    let mut read = [0u8; 8];
    let mut write = [0u8; 8];
    read[0] = 0b0000_0110; // 1 and 2
    write[0] = 0b0010_0000; // 5
    let open = |fd: u64| fd < 4;
    assert_eq!(first_closed(&[&read, &write], 64, open), Some(5));
    assert_eq!(first_closed(&[&read], 64, open), None);
    // Only what nfds covers is looked at.
    assert_eq!(first_closed(&[&read, &write], 5, open), None);
}

#[test]
fn select_narrows_a_set_to_what_is_ready_and_counts_it() {
    let mut set = [0b0010_0110u8, 0, 0, 0, 0, 0, 0, 0];
    let kept = narrow(&mut set, 64, |fd| if fd == 2 { 1 } else { 0 }, 1);
    assert_eq!(kept, 1);
    assert_eq!(set[0], 0b0000_0100);
    // A set shorter than nfds is narrowed as far as it goes, never past it.
    let mut short = [0xffu8; 2];
    assert_eq!(narrow(&mut short, 256, |_| 1, 1), 16);
    assert!(is_set(&short, 15) && !is_set(&short, 16));
}
