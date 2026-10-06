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

//! The vfs handle table: one owner holds at most half of it, and the handles
//! of an owner that ended are closed while a living owner's and the kernel's
//! own stay open.

use crate::vfs_store::{Store, StoreError, PER_OWNER_FDS};

const A: u32 = 40;
const B: u32 = 41;
const C: u32 = 42;
const KERNEL: u32 = 0;

fn open(s: &mut Store, path: &str, pid: u32) -> Result<u32, StoreError> {
    s.open(path, pid, true, false, false, true)
}

fn fill(s: &mut Store, pid: u32) -> Vec<u32> {
    (0..PER_OWNER_FDS).map(|_| open(s, "/data/f", pid).expect("within the share")).collect()
}

#[test]
fn an_owner_stops_at_its_share_and_another_still_opens() {
    let mut s = Store::new();
    fill(&mut s, A);
    assert_eq!(s.held_by(A), PER_OWNER_FDS);
    assert_eq!(open(&mut s, "/data/f", A), Err(StoreError::Full), "past its share");
    assert!(open(&mut s, "/data/f", B).is_ok(), "another owner still opens");
}

#[test]
fn an_owner_at_its_share_creates_and_truncates_nothing() {
    let mut s = Store::new();
    let fd = open(&mut s, "/data/keep", A).expect("open");
    s.write(fd, A, b"kept").expect("write");
    for _ in 1..PER_OWNER_FDS {
        open(&mut s, "/data/keep", A).expect("within the share");
    }
    assert_eq!(s.open("/data/keep", A, false, true, false, true), Err(StoreError::Full));
    assert_eq!(open(&mut s, "/data/new", A), Err(StoreError::Full));
    let r = open(&mut s, "/data/keep", B).expect("B opens");
    assert_eq!(s.read(r, B, 16), Ok(b"kept".to_vec()), "the refused truncate changed nothing");
    assert_eq!(s.open("/data/new", B, false, false, false, true), Err(StoreError::NotFound));
}

/*
 * A client that ended without closing kept its handles for good: two such
 * clients and no program could open a file again until a reboot.
 */
#[test]
fn the_handles_of_an_ended_owner_are_closed_and_its_place_reused() {
    let mut s = Store::new();
    let a = fill(&mut s, A);
    fill(&mut s, B);
    assert_eq!(open(&mut s, "/data/f", C), Err(StoreError::Full), "the table is full");
    assert_eq!(s.close_ended(|pid| pid != A), PER_OWNER_FDS);
    assert_eq!(s.held_by(A), 0);
    assert_eq!(s.close(a[0], A), Err(StoreError::BadFd), "A's handle is gone");
    assert_eq!(s.held_by(B), PER_OWNER_FDS, "B's handles stay open");
    assert!(open(&mut s, "/data/f", C).is_ok(), "a new owner is served");
}

#[test]
fn nobody_ended_closes_nothing() {
    let mut s = Store::new();
    let fd = open(&mut s, "/data/f", A).expect("open");
    assert_eq!(s.close_ended(|_| true), 0);
    assert!(s.close(fd, A).is_ok(), "A's handle is still open");
}

/*
 * mk_pid_alive never names pid 0 as alive, and 0 is the kernel speaking for
 * itself: taking its handles would close files under the kernel's feet.
 */
#[test]
fn the_kernel_s_own_handles_are_never_closed() {
    let mut s = Store::new();
    let k = open(&mut s, "/data/f", KERNEL).expect("open");
    let a = open(&mut s, "/data/f", A).expect("open");
    assert_eq!(s.close_ended(|_| false), 1, "only A's handle");
    assert!(s.close(k, KERNEL).is_ok(), "the kernel's handle is still open");
    assert_eq!(s.close(a, A), Err(StoreError::BadFd));
}
