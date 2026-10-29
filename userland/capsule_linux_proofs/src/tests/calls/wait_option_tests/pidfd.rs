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

//! A pidfd reads POLLIN only once its child has ended, and a waitid on it
//! reaps that child and no other, from its parent's lists.

use crate::linux::abi::errno::EBADF;
use crate::linux::call::pidfd_life::{bits, kept, life, wait_for, Life, POLLHUP, POLLIN};
use crate::linux::call::wait_opts::{WALL, WEXITED, WNOHANG, WNOWAIT};
use crate::linux::call::wait_take::{take, Found};

const PARENT: u32 = 40;
const CHILD: u32 = 41;

/// The pidfd for `pid` once the family's note has run: what it keeps, and
/// what poll reads on it.
fn noted(pid: u32, live: &[u32], zombies: &[u32]) -> (u32, u16) {
    let (pid, ended) = kept(pid, life(pid, live, zombies));
    (pid, bits(pid, ended))
}

#[test]
fn a_pidfd_reads_pollin_only_after_its_child_exits() {
    let (pid, running) = noted(CHILD, &[PARENT, CHILD], &[]);
    assert_eq!((pid, running), (CHILD, 0), "nothing while the child runs");
    let (pid, ended) = noted(pid, &[PARENT], &[CHILD]);
    assert_eq!((pid, ended & (POLLIN | POLLHUP)), (CHILD, POLLIN), "a zombie");
    let (pid, reaped) = noted(pid, &[PARENT], &[]);
    assert_eq!((pid, reaped & (POLLIN | POLLHUP)), (0, POLLIN | POLLHUP), "reaped");
    // A later process given the child's number is not the pidfd's.
    assert_eq!(noted(pid, &[PARENT, CHILD], &[]).1 & POLLHUP, POLLHUP);
}

#[test]
fn waitid_on_a_pidfd_reaps_its_child_and_no_other() {
    // Three children: 50 runs, 51 has ended, and the pidfd's ended with 3.
    let (mut ended, mut children) = (vec![(51, 0), (CHILD, 3 << 8)], vec![50, 51, CHILD]);
    let (pid, options, eagain) = wait_for(Some((CHILD, false)), WEXITED | WALL).unwrap();
    assert_eq!((pid, options, eagain), (CHILD, WEXITED | WALL, false));
    let peek = take(&mut ended, &mut children, options | WNOWAIT, |p| p == pid);
    assert_eq!((peek, children.len()), (Found::Ended(CHILD, 3 << 8), 3), "WNOWAIT only looks");
    let reaped = take(&mut ended, &mut children, options, |p| p == pid);
    assert_eq!(reaped, Found::Ended(CHILD, 3 << 8));
    assert_eq!((ended, children.clone()), (vec![(51, 0)], vec![50, 51]));
    // Reaped, its pidfd reads gone, and a second waitid on it finds no child.
    assert_eq!(life(pid, &[PARENT, 50], &[51]), Life::Gone);
    let (gone, _) = kept(pid, Life::Gone);
    let (pid, options, _) = wait_for(Some((gone, false)), WEXITED).unwrap();
    assert_eq!(take(&mut vec![(51, 0)], &mut children, options, |p| p == pid), Found::NoChild);
}

#[test]
fn a_nonblocking_pidfd_never_parks() {
    let blocking = wait_for(Some((CHILD, true)), WEXITED);
    assert_eq!(blocking, Ok((CHILD, WEXITED | WNOHANG, true)), "EAGAIN, not 0");
    let asked = wait_for(Some((CHILD, true)), WEXITED | WNOHANG);
    assert_eq!(asked, Ok((CHILD, WEXITED | WNOHANG, false)), "0, as WNOHANG answers");
    let found = take(&mut vec![], &mut vec![CHILD], WEXITED | WNOHANG, |p| p == CHILD);
    assert_eq!(found, Found::NotYet);
    assert_eq!(wait_for(None, WEXITED), Err(EBADF), "no pidfd");
}
