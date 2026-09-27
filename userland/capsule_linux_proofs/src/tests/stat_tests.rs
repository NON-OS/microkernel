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


//! The struct a libc reads out of fstat.

use crate::statbuf::{build, S_IFDIR, S_IFREG, STAT_LEN};

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

fn u64_at(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
}

/// Offsets are the x86_64 layout: nlink at 16, mode at 24, size at 48, blksize
/// at 56, blocks at 64.
#[test]
fn a_regular_file_lands_in_the_right_fields() {
    let s = build(4096, false, 7);
    assert_eq!(s.len(), STAT_LEN);
    assert_eq!(u64_at(&s, 16), 1);
    assert_eq!(u32_at(&s, 24), S_IFREG | 0o644);
    assert_eq!(u64_at(&s, 48), 4096);
    assert_eq!(u64_at(&s, 56), 4096);
    assert_eq!(u64_at(&s, 64), 8);
}

#[test]
fn a_directory_says_so_in_the_mode() {
    let s = build(0, true, 7);
    assert_eq!(u32_at(&s, 24), S_IFDIR | 0o755);
    assert_eq!(u64_at(&s, 48), 0);
}

#[test]
fn block_count_rounds_up_to_the_next_five_hundred_and_twelve() {
    assert_eq!(u64_at(&build(1, false, 7), 64), 1);
    assert_eq!(u64_at(&build(512, false, 7), 64), 1);
    assert_eq!(u64_at(&build(513, false, 7), 64), 2);
}

#[test]
fn everything_unknown_is_left_at_zero() {
    let s = build(10, false, 7);
    // st_ino at 8 is known now: `the_inode_given_is_the_inode_reported`.
    for at in [0, 40, 72, 88, 104] {
        assert_eq!(u64_at(&s, at), 0, "offset {at} should be untouched");
    }
}

#[test]
fn the_inode_given_is_the_inode_reported() {
    // st_ino sits after st_dev, at byte 8. Every file used to report 0, and
    // musl's loader took two libraries with one inode for the same file.
    assert_eq!(u64_at(&build(10, false, 0xdead_beef), 8), 0xdead_beef);
    assert_ne!(u64_at(&build(10, false, 1), 8), u64_at(&build(10, false, 2), 8));
}
