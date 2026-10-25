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

//! What copy_process refuses, and what cannot be made as asked.

use super::super::super::random::Regs;
use super::made::{
    thread, DETACHED, FILES, FS, NEWNET, NEWNS, NEWPID, NEWUSER, NEWUTS, PIDFD, SIGCHLD, SIGHAND,
    THREAD, VFORK, VM,
};
use crate::linux::abi::errno::{EINVAL, ENOSYS, EPERM};
use crate::linux::call::clone_flags::{clone_valid, flags_of};

#[test]
fn what_copy_process_refuses_is_refused_with_its_errno() {
    let base = VM | FS | FILES | SIGHAND | THREAD;
    let table: [(u64, i64); 10] = [
        (VM | FS | FILES | THREAD, EINVAL),
        (FS | FILES | SIGHAND, EINVAL),
        (FS | NEWNS | SIGCHLD, EINVAL),
        (FS | NEWUSER | SIGCHLD, EINVAL),
        (base & !FS | NEWUSER, EINVAL),
        (base | NEWPID, EINVAL),
        (base | PIDFD, EINVAL),
        (PIDFD | DETACHED | SIGCHLD, EINVAL),
        (NEWNET | SIGCHLD, EPERM),
        (base | NEWUTS, EPERM),
    ];
    for (flags, want) in table {
        assert_eq!(clone_valid(flags), Err(want), "{flags:#x}");
    }
}

#[test]
fn a_thread_that_cannot_be_made_as_asked_is_enosys() {
    let base = VM | FS | FILES | SIGHAND | THREAD;
    // Its own descriptors, or its own working directory, which no thread
    // here has; and a creator left parked as vfork parks it.
    assert_eq!(thread(base & !FILES), Err(ENOSYS));
    assert_eq!(thread(base & !FS), Err(ENOSYS));
    assert_eq!(thread(base | VFORK), Err(ENOSYS));
}

#[test]
fn random_flags_never_make_a_thread_without_what_a_thread_shares() {
    let mut r = Regs::new(0x636c_6f6e_655f_6667);
    for _ in 0..200_000 {
        let shared = r.small(2) * (VM | FS | FILES | SIGHAND | THREAD);
        let flags = flags_of((r.any() & r.any()) | shared);
        if clone_valid(flags).is_ok() {
            assert!(flags & THREAD == 0 || flags & SIGHAND != 0);
            assert!(flags & SIGHAND == 0 || flags & VM != 0);
            assert_eq!(flags & (NEWNS | NEWUTS | NEWUSER | NEWPID | NEWNET), 0);
        }
        if thread(flags).is_ok() {
            assert_eq!(
                flags & (VM | FS | FILES | SIGHAND | THREAD | VFORK),
                VM | FS | FILES | SIGHAND | THREAD
            );
        }
    }
}
