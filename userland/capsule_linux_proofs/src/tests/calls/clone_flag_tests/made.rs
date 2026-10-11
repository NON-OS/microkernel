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

//! The flags, every thread the C and Go runtimes ask for, and the legacy call.

use crate::linux::call::clone_flags::{clone_valid, flags_of, thread_served};

pub(super) const VM: u64 = 0x100;
pub(super) const FS: u64 = 0x200;
pub(super) const FILES: u64 = 0x400;
pub(super) const SIGHAND: u64 = 0x800;
pub(super) const PIDFD: u64 = 0x1000;
pub(super) const VFORK: u64 = 0x4000;
pub(super) const THREAD: u64 = 0x1_0000;
pub(super) const NEWNS: u64 = 0x2_0000;
const SYSVSEM: u64 = 0x4_0000;
const SETTLS: u64 = 0x8_0000;
const PARENT_SETTID: u64 = 0x10_0000;
const CHILD_CLEARTID: u64 = 0x20_0000;
pub(super) const DETACHED: u64 = 0x40_0000;
const CHILD_SETTID: u64 = 0x100_0000;
pub(super) const NEWUTS: u64 = 0x400_0000;
pub(super) const NEWUSER: u64 = 0x1000_0000;
pub(super) const NEWPID: u64 = 0x2000_0000;
pub(super) const NEWNET: u64 = 0x4000_0000;
pub(super) const SIGCHLD: u64 = 17;

pub(super) fn thread(flags: u64) -> Result<(), i64> {
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
fn the_legacy_call_reads_only_the_low_half_of_its_flags() {
    let base = VM | FS | FILES | SIGHAND | THREAD;
    assert_eq!(flags_of(base | 1 << 40), base);
    assert_eq!(thread(flags_of(base | NEWNET << 32)), Ok(()));
}
