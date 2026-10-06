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

//! What the vfs store may hold (`capsule_vfs/src/store/fdtable/budget.rs`).
//! Every request that adds bytes or names is refused as Full past the budget,
//! and the store, with every other client's files, goes on.

use crate::vfs_store::{Store, StoreError, DATA_BYTES_MAX, NAMES_PER_OWNER};

const A: u32 = 40;
const B: u32 = 41;
const KERNEL: u32 = 0;
const MIB: usize = 1 << 20;

fn file(s: &mut Store, path: &str, pid: u32) -> u32 {
    s.open(path, pid, true, false, false, true).expect("opens")
}

/// Create `path` and let its handle go, so only the name stays.
fn name(s: &mut Store, path: &str, pid: u32) -> Result<(), StoreError> {
    let fd = s.open(path, pid, true, false, false, true)?;
    s.close(fd, pid)
}

fn data_held(s: &Store) -> usize {
    s.usage().1 as usize
}

/// Write `chunk` bytes at a time to a new file until refused; bytes written.
fn fill(s: &mut Store, path: &str, pid: u32, chunk: usize) -> usize {
    let fd = file(s, path, pid);
    let piece = vec![0xA5u8; chunk];
    let mut written = 0;
    while let Ok(n) = s.write(fd, pid, &piece) {
        written += n as usize;
        assert!(written <= 64 * MIB, "a file passed the per-file ceiling");
    }
    written
}

#[test]
fn writes_stop_at_the_budget_and_the_store_goes_on() {
    let mut s = Store::new();
    let mut i = 0;
    loop {
        let wrote = fill(&mut s, &format!("/tmp/big{i}"), A, 4 * MIB);
        i += 1;
        if wrote < 64 * MIB {
            break;
        }
        assert!(i < 16, "the budget never refused");
    }
    assert!(s.held_except(None) <= DATA_BYTES_MAX);
    // Refused, not ended: the store still answers and still serves small files.
    let fd = file(&mut s, "/tmp/more", A);
    assert_eq!(s.write(fd, A, &vec![1; 64 * MIB]), Err(StoreError::Full));
    // Overwriting bytes already held needs no room and is still served.
    let fd = file(&mut s, "/tmp/big0", A);
    assert!(s.write(fd, A, &vec![1; 64 * MIB]).is_ok());
    let fd = file(&mut s, "/tmp/small", B);
    assert!(s.write(fd, B, b"still here").is_ok() || data_held(&s) + 4096 > DATA_BYTES_MAX);
}

#[test]
fn the_allocation_never_passes_the_budget_under_small_writes() {
    let mut s = Store::new();
    let mut i = 0;
    while fill(&mut s, &format!("/tmp/f{i}"), A, 4096) == 64 * MIB {
        i += 1;
        assert!(i < 16);
    }
    // What is allocated, not only what is written, stays inside the budget.
    assert!(s.held_except(None) <= DATA_BYTES_MAX);
    assert!(data_held(&s) <= s.held_except(None));
}

#[test]
fn a_copy_past_the_budget_is_refused_whole() {
    let mut s = Store::new();
    for i in 0..2 {
        assert_eq!(fill(&mut s, &format!("/tmp/c{i}"), A, 8 * MIB), 64 * MIB);
    }
    let before = (s.usage().0, data_held(&s));
    // A third 64 MiB file does not fit beside two in 160 MiB.
    assert_eq!(s.copy("/tmp/c0", "/tmp/c2", false, A), Err(StoreError::Full));
    assert_eq!((s.usage().0, data_held(&s)), before);
}

#[test]
fn a_truncate_that_grows_past_the_budget_is_refused() {
    let mut s = Store::new();
    for i in 0..2 {
        assert_eq!(fill(&mut s, &format!("/tmp/t{i}"), A, 8 * MIB), 64 * MIB);
    }
    file(&mut s, "/tmp/t2", A);
    assert_eq!(s.truncate("/tmp/t2", (40 * MIB) as u64), Err(StoreError::Full));
    assert!(s.truncate("/tmp/t2", MIB as u64).is_ok());
}

#[test]
fn one_owner_creates_at_most_its_names() {
    let mut s = Store::new();
    let base = s.names_of(A);
    let mut made = 0;
    while name(&mut s, &format!("/tmp/n{made}"), A).is_ok() {
        made += 1;
        assert!(made <= NAMES_PER_OWNER);
    }
    assert_eq!(base + made, NAMES_PER_OWNER);
    assert_eq!(s.mkdir("/tmp/dir_a", A), Err(StoreError::Full));
    // Another owner still creates.
    assert!(s.open("/tmp/other", B, true, false, false, true).is_ok());
    assert!(s.mkdir("/tmp/dir_b", B).is_ok());
}

#[test]
fn the_kernel_is_not_held_to_a_name_share() {
    let mut s = Store::new();
    for i in 0..NAMES_PER_OWNER + 8 {
        assert!(s.mkdir(&format!("/k{i}"), KERNEL).is_ok(), "kernel dir {i}");
    }
}

#[test]
fn a_tree_copy_past_the_name_share_adds_nothing() {
    let mut s = Store::new();
    assert!(s.mkdir("/src", B).is_ok());
    for i in 0..8 {
        name(&mut s, &format!("/src/f{i}"), B).expect("source");
    }
    while s.names_of(A) + 4 < NAMES_PER_OWNER {
        let n = s.names_of(A);
        name(&mut s, &format!("/tmp/a{n}"), A).expect("within the share");
    }
    let before = s.usage().0;
    assert_eq!(s.copy("/src", "/dst", true, A), Err(StoreError::Full));
    assert_eq!(s.usage().0, before);
}

#[test]
fn installing_bytes_is_held_to_the_budget_too() {
    let mut s = Store::new();
    for i in 0..2 {
        assert_eq!(fill(&mut s, &format!("/tmp/i{i}"), A, 8 * MIB), 64 * MIB);
    }
    let big = vec![7u8; 40 * MIB];
    assert_eq!(s.install_bytes("/pkg/x", 0, &big, A), Err(StoreError::Full));
    assert!(s.install_bytes("/pkg/y", 0, b"small", A).is_ok());
}
