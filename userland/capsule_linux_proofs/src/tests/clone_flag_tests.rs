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

//! clone refuses what copy_process refuses, with its errno; makes every
//! thread the C and Go runtimes ask for; and answers ENOSYS, never a thread
//! that shares what it asked to keep, for the rest.

use super::random::Regs;
use crate::linux::abi::errno::{EINVAL, ENOSYS, EPERM};
use crate::linux::call::clone_flags::{clone_valid, flags_of, thread_served};

const VM: u64 = 0x100;
const FS: u64 = 0x200;
const FILES: u64 = 0x400;
const SIGHAND: u64 = 0x800;
const PIDFD: u64 = 0x1000;
const VFORK: u64 = 0x4000;
const THREAD: u64 = 0x1_0000;
const NEWNS: u64 = 0x2_0000;
const SYSVSEM: u64 = 0x4_0000;
const SETTLS: u64 = 0x8_0000;
const PARENT_SETTID: u64 = 0x10_0000;
const CHILD_CLEARTID: u64 = 0x20_0000;
const DETACHED: u64 = 0x40_0000;
const CHILD_SETTID: u64 = 0x100_0000;
const NEWUTS: u64 = 0x400_0000;
const NEWUSER: u64 = 0x1000_0000;
const NEWPID: u64 = 0x2000_0000;
const NEWNET: u64 = 0x4000_0000;
const SIGCHLD: u64 = 17;

fn thread(flags: u64) -> Result<(), i64> {
    clone_valid(flags).and_then(|()| thread_served(flags))
}

#[test]
fn every_runtime_thread_is_made() {
    let base = VM | FS | FILES | SIGHAND | THREAD | SYSVSEM;
    let musl = base | SETTLS | PARENT_SETTID | CHILD_CLEARTID | DETACHED;
    let glibc = base | SETTLS | PARENT_SETTID | CHILD_CLEARTID;
    let go = base;
    for flags in [musl, glibc, go] {
        assert_eq!(thread(flags), Ok(()), "{flags:#x}");
    }
    // fork's clone, and posix_spawn's.
    assert_eq!(clone_valid(CHILD_SETTID | CHILD_CLEARTID | SIGCHLD), Ok(()));
    assert_eq!(clone_valid(VM | VFORK | SIGCHLD), Ok(()));
}

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
fn the_legacy_call_reads_only_the_low_half_of_its_flags() {
    let base = VM | FS | FILES | SIGHAND | THREAD;
    assert_eq!(flags_of(base | 1 << 40), base);
    assert_eq!(thread(flags_of(base | NEWNET << 32)), Ok(()));
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
