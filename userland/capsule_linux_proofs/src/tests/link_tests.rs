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

//! A hard link here is a copy, so it is held to Linux's rule that a link
//! never crosses mounts: from the store's read-only tree into a private
//! directory it is EXDEV, not a file as large as anything installed put in
//! the shared store with no quota asked.

use crate::linux::abi::errno::EXDEV;
use crate::linux::file::link::across::same_mount;
use crate::linux::file::mounts::MOUNTS;

#[test]
fn a_link_within_one_mount_is_taken() {
    assert_eq!(same_mount(b"/tmp/a", b"/tmp/b"), Ok(()));
    assert_eq!(same_mount(b"/tmp/a", b"/tmp/sub/b"), Ok(()));
    assert_eq!(same_mount(b"/home/u/a", b"/home/b"), Ok(()));
    assert_eq!(same_mount(b"/usr/bin/a", b"/etc/b"), Ok(()), "the tree is one mount");
}

#[test]
fn a_link_out_of_the_tree_into_a_private_directory_is_exdev() {
    assert_eq!(same_mount(b"/usr/lib/libc.so", b"/tmp/x"), Err(EXDEV));
    assert_eq!(same_mount(b"/bin/busybox", b"/home/user/sh"), Err(EXDEV));
    assert_eq!(same_mount(b"/tmp/x", b"/usr/lib/x"), Err(EXDEV));
}

#[test]
fn a_link_from_one_tmpfs_to_another_is_exdev() {
    assert_eq!(same_mount(b"/tmp/a", b"/home/a"), Err(EXDEV));
    assert_eq!(same_mount(b"/dev/shm/a", b"/dev/a"), Err(EXDEV));
    assert_eq!(same_mount(b"/var/tmp/a", b"/var/a"), Err(EXDEV));
    assert_eq!(same_mount(b"/tmpx/a", b"/tmp/a"), Err(EXDEV), "a prefix is not the mount");
}

/// Every writable mount against every other mount and the tree: only a
/// mount with itself takes a link.
#[test]
fn every_pair_of_mounts() {
    for a in MOUNTS {
        for b in MOUNTS {
            let (from, to) = (format!("{}/f", a.2), format!("{}/g", b.2));
            let want = if a.0 == b.0 { Ok(()) } else { Err(EXDEV) };
            assert_eq!(same_mount(from.as_bytes(), to.as_bytes()), want, "{from} to {to}");
        }
    }
}
