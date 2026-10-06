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

//! The ramfs handle table: one caller holds at most half of it, and the
//! handles of a caller that ended are closed while a living caller's and the
//! kernel's own stay open.

use ramfs_host::handles::{HandleTable, MAX_HANDLES, PER_OWNER};

const A: u32 = 90;
const B: u32 = 91;
const C: u32 = 92;
const KERNEL: u32 = 0;

fn fill(t: &mut HandleTable, owner: u32) -> Vec<u64> {
    (0..PER_OWNER).map(|_| t.insert("/ram/f".into(), owner).expect("within the share")).collect()
}

#[test]
#[allow(clippy::assertions_on_constants)] // the relation is the point
fn one_caller_never_holds_every_handle() {
    assert!(PER_OWNER < MAX_HANDLES);
}

#[test]
fn a_caller_stops_at_its_share_and_the_kernel_still_opens() {
    let mut t = HandleTable::new();
    fill(&mut t, A);
    assert_eq!(t.held_by(A), PER_OWNER);
    assert!(t.insert("/ram/f".into(), A).is_none(), "past its share");
    assert!(t.insert("/ram/f".into(), KERNEL).is_some(), "the kernel's /ram files still open");
}

/*
 * Only the owner or the kernel closes a handle, so a caller that ended
 * without closing kept its handles for good, and two such callers left the
 * kernel unable to open any process's /ram file.
 */
#[test]
fn the_handles_of_an_ended_caller_are_closed_and_its_places_reused() {
    let mut t = HandleTable::new();
    let a = fill(&mut t, A);
    let b = fill(&mut t, B);
    assert!(t.is_full());
    assert!(t.insert("/ram/f".into(), C).is_none(), "the table is full");
    assert_eq!(t.close_ended(|pid| pid != A), PER_OWNER);
    assert_eq!(t.held_by(A), 0);
    assert!(t.path_for(a[0], A).is_err(), "A's handle is gone");
    assert!(b.iter().all(|&h| t.path_for(h, B).is_ok()), "B's stay open");
    assert!(t.insert("/ram/f".into(), C).is_some(), "a new caller is served");
    assert_eq!(t.close_ended(|_| true), 0, "nobody ended, nothing closed");
}

/*
 * mk_pid_alive never names pid 0 as alive, and 0 is the kernel, which opens
 * every process's /ram file: taking its handles would close them all.
 */
#[test]
fn the_kernel_s_own_handles_are_never_closed() {
    let mut t = HandleTable::new();
    let k = t.insert("/ram/f".into(), KERNEL).expect("open");
    let a = t.insert("/ram/f".into(), A).expect("open");
    assert_eq!(t.close_ended(|_| false), 1, "only A's handle");
    assert!(t.path_for(k, KERNEL).is_ok(), "the kernel's handle is still open");
    assert!(t.path_for(a, KERNEL).is_err());
}
