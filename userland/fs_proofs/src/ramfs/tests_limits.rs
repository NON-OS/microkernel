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

//! A file grows only as far as the capsule's heap can carry it. A write at
//! an offset past the largest file, or a truncate to a length past it, used
//! to resize a buffer to that size: capacity overflow, or an allocation the
//! heap could not make, and ramfs stopped with every process's /ram in it.

use ramfs_host::store::limits::{MAX_FILES, MAX_FILE_BYTES, MAX_STORE_BYTES};
use ramfs_host::Ramfs;

use super::wire::{open, open_status, read_all, truncate, write};

const EFBIG: i32 = -27;
const ENOSPC: i32 = -28;

#[test]
fn a_write_past_the_largest_file_is_refused_and_changes_nothing() {
    let mut fs = Ramfs::new();
    let h = open(&mut fs, "/ram/a");
    assert_eq!(write(&mut fs, h, 0, b"hello"), 5);
    let max = MAX_FILE_BYTES as u64;
    for offset in [u64::MAX - 1, u64::MAX, 1 << 40, max, max - 2] {
        assert_eq!(write(&mut fs, h, offset, b"xyz"), EFBIG, "offset {offset}");
    }
    assert_eq!(read_all(&mut fs, h), b"hello");
    assert_eq!(write(&mut fs, h, max - 3, b"end"), 3, "the largest file is allowed");
    assert_eq!(read_all(&mut fs, h).len(), MAX_FILE_BYTES);
}

#[test]
fn a_truncate_past_the_largest_file_is_refused_and_changes_nothing() {
    let mut fs = Ramfs::new();
    let h = open(&mut fs, "/ram/a");
    assert_eq!(write(&mut fs, h, 0, b"hello"), 5);
    for len in [u64::MAX, 1 << 40, MAX_FILE_BYTES as u64 + 1] {
        assert_eq!(truncate(&mut fs, h, len), EFBIG, "length {len}");
    }
    assert_eq!(read_all(&mut fs, h), b"hello");
    assert_eq!(truncate(&mut fs, h, MAX_FILE_BYTES as u64), 0);
    assert_eq!(truncate(&mut fs, h, 2), 0);
    assert_eq!(read_all(&mut fs, h), b"he");
}

#[test]
fn the_store_refuses_bytes_past_its_total_and_keeps_what_it_has() {
    let mut fs = Ramfs::new();
    let files = MAX_STORE_BYTES / MAX_FILE_BYTES;
    let full = vec![0x5Au8; MAX_FILE_BYTES];
    let handles: Vec<u64> = (0..=files).map(|i| open(&mut fs, &format!("/ram/f{i}"))).collect();
    for &h in &handles[..files] {
        assert_eq!(write(&mut fs, h, 0, &full[..4096]), 4096);
        assert_eq!(truncate(&mut fs, h, MAX_FILE_BYTES as u64), 0);
    }
    let last = handles[files];
    assert_eq!(write(&mut fs, last, 0, b"one more"), ENOSPC);
    assert_eq!(truncate(&mut fs, last, 1), ENOSPC);
    assert_eq!(read_all(&mut fs, last), b"");
    assert_eq!(truncate(&mut fs, handles[0], 0), 0, "shrinking is always allowed");
    assert_eq!(write(&mut fs, last, 0, b"one more"), 8, "and makes room");
    assert_eq!(read_all(&mut fs, handles[1]).len(), MAX_FILE_BYTES);
}

#[test]
fn the_store_refuses_files_past_its_count() {
    let mut fs = Ramfs::new();
    for i in 0..MAX_FILES {
        let h = open(&mut fs, &format!("/ram/{i}"));
        assert_eq!(super::wire::call(&mut fs, super::wire::CLOSE, &h.to_le_bytes()).0, 0);
    }
    assert_eq!(open_status(&mut fs, "/ram/one-more"), ENOSPC);
    assert_eq!(open_status(&mut fs, "/ram/7"), 0, "an existing file still opens");
}
