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

//! A Linux run that crashed leaves its private files under
//! /linux-private/<id>; vfs removes those of an owner that ended
//! (`capsule_vfs/src/store/fdtable/reap.rs`).

use crate::vfs_store::Store;

const DEAD: u32 = 40;
const LIVE: u32 = 41;
const OTHER: u32 = 42;
const KERNEL: u32 = 0;

fn alive(pid: u32) -> bool {
    pid != DEAD
}

fn put(s: &mut Store, path: &str, pid: u32, bytes: &[u8]) -> u32 {
    let fd = s.open(path, pid, true, false, false, true).expect("opens");
    s.write(fd, pid, bytes).expect("writes");
    fd
}

fn exists(s: &Store, path: &str) -> bool {
    s.stat(path).is_ok()
}

#[test]
fn an_ended_runs_private_files_go_and_the_rest_stay() {
    let mut s = Store::new();
    // The first run's mkdir -p made the shared parent too, as it does live.
    s.mkdir("/linux-private/7", DEAD).expect("dead run");
    put(&mut s, "/linux-private/7/tmp.db", DEAD, b"secret");
    s.mkdir("/linux-private/8", LIVE).expect("live run");
    put(&mut s, "/linux-private/8/notes", LIVE, b"mine");
    put(&mut s, "/home/dead.txt", DEAD, b"kept");
    let dropped = s.drop_private_of_ended(alive);
    assert_eq!(dropped, 2);
    assert!(!exists(&s, "/linux-private/7/tmp.db"));
    assert!(!exists(&s, "/linux-private/7"));
    assert!(exists(&s, "/linux-private/8/notes"));
    assert!(exists(&s, "/linux-private"));
    assert!(exists(&s, "/home/dead.txt"), "only private files are taken");
    assert_eq!(s.drop_private_of_ended(alive), 0);
}

#[test]
fn another_clients_handle_still_reads_its_own_file() {
    let mut s = Store::new();
    s.mkdir("/linux-private/7", DEAD).expect("dead run");
    for i in 0..5 {
        put(&mut s, &format!("/linux-private/7/f{i}"), DEAD, b"gone");
    }
    let fd = put(&mut s, "/data/after", OTHER, b"0123456789");
    s.drop_private_of_ended(alive);
    assert!(s.seek(fd, OTHER, crate::vfs_store::SeekWhence::Set, 0).is_ok());
    assert_eq!(s.read(fd, OTHER, 64).expect("reads"), b"0123456789");
}

#[test]
fn the_kernels_private_files_are_never_taken() {
    let mut s = Store::new();
    s.mkdir("/linux-private/k", KERNEL).expect("kernel");
    put(&mut s, "/linux-private/k/x", KERNEL, b"k");
    put(&mut s, "/linux-private/live", LIVE, b"l");
    assert_eq!(s.drop_private_of_ended(|_| false), 1);
    assert!(exists(&s, "/linux-private/k/x"));
    assert!(!exists(&s, "/linux-private/live"));
}
