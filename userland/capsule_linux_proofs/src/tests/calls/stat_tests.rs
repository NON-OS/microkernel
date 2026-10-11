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

/* The struct a libc reads out of fstat. */

use crate::statbuf::{blocks, build, inode, Meta, STAT_LEN};

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

fn u64_at(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
}

fn file() -> Meta {
    let (mode, size, ino, nlink, rdev, dev) = (0o100640, 5000, 0xdead_beef, 2, 0x0103, 9);
    Meta { mode, size, ino, nlink, rdev, dev, mtime_ms: 1_234_567, atime_ms: 7_000_001 }
}

/*
 * x86_64: dev 0, ino 8, nlink 16, mode 24, uid/gid 28/32, rdev 40, size 48, blksize 56, blocks 64.
 */
#[test]
fn each_field_lands_where_x86_64_linux_has_it() {
    let s = build(&file());
    assert_eq!(s.len(), STAT_LEN);
    assert_eq!(u64_at(&s, 0), 9);
    assert_eq!(u64_at(&s, 8), 0xdead_beef);
    assert_eq!(u64_at(&s, 16), 2);
    assert_eq!(u32_at(&s, 24), 0o100640);
    assert_eq!((u32_at(&s, 28), u32_at(&s, 32)), (0, 0), "owner and group are root");
    assert_eq!(u64_at(&s, 40), 0x0103);
    assert_eq!(u64_at(&s, 48), 5000);
    assert_eq!(u64_at(&s, 56), 4096);
    assert_eq!(u64_at(&s, 64), 16);
}

/* atime at 72, mtime at 88 and ctime at 104, each seconds then nanoseconds. */
#[test]
fn times_are_split_into_seconds_and_nanoseconds() {
    let s = build(&file());
    assert_eq!((u64_at(&s, 72), u64_at(&s, 80)), (7000, 1_000_000));
    assert_eq!((u64_at(&s, 88), u64_at(&s, 96)), (1234, 567_000_000));
    assert_eq!((u64_at(&s, 104), u64_at(&s, 112)), (1234, 567_000_000));
}

/* tmpfs counts whole pages, in 512-byte units. */
#[test]
fn blocks_are_whole_pages() {
    assert_eq!([blocks(0), blocks(1), blocks(4096), blocks(4097)], [0, 8, 8, 16]);
}

/* Every file once reported inode 0, and musl's loader took two libraries for one file. */
#[test]
fn an_inode_is_never_zero_and_differs_by_path() {
    assert_ne!(inode(b"/lib/a.so"), inode(b"/lib/b.so"));
    assert_eq!(inode(b"/"), inode(b"/"));
    assert_ne!(inode(b""), 0);
}
