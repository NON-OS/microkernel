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

//! Calls refused on purpose, each with its reason. NONOS has its own
//! isolation model, and these would import Linux's: they are decisions, so
//! they are said as decisions and never read as forgotten.

use crate::linux::abi::errno;

const PERM: i64 = errno::EPERM;

const REFUSED: &[(u64, i64, &str)] = &[
    (101, PERM, "ptrace: a guest does not inspect or steer another"),
    (310, PERM, "process_vm_readv: no guest reads another's memory"),
    (311, PERM, "process_vm_writev: no guest writes another's memory"),
    (125, PERM, "capget: capabilities are the kernel's, not Linux's"),
    (126, PERM, "capset: capabilities are the kernel's, not Linux's"),
    (165, PERM, "mount: the tree is laid out by the personality"),
    (166, PERM, "umount2: the tree is laid out by the personality"),
    (161, PERM, "chroot: the family is already rooted at /linux"),
    (272, PERM, "unshare: namespaces are the personality's"),
    (308, PERM, "setns: namespaces are the personality's"),
    (425, errno::ENOSYS, "io_uring_setup: a second call path around the gate"),
    (426, errno::ENOSYS, "io_uring_enter: a second call path around the gate"),
    (427, errno::ENOSYS, "io_uring_register: a second call path around the gate"),
    (253, errno::ENOSYS, "inotify_init: the store sends no change events to watch"),
    (294, errno::ENOSYS, "inotify_init1: the store sends no change events to watch"),
    (254, errno::ENOSYS, "inotify_add_watch: the store sends no change events to watch"),
    (255, errno::ENOSYS, "inotify_rm_watch: the store sends no change events to watch"),
];

/// The errno for a call refused on purpose, after saying why; None otherwise.
pub fn refused(number: u64) -> Option<u64> {
    let (_, code, why) = REFUSED.iter().find(|(nr, _, _)| *nr == number)?;
    let line = alloc::format!("[LINUX] refused {why}\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    Some(errno::fail(*code))
}
