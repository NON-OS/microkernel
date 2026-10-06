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

//! The clone flags Linux refuses whoever asks, and the ones a thread here
//! may be made with. Pure, so the host proofs hold each refusal and its
//! errno.

use crate::linux::abi::errno;

const CLONE_VM: u64 = 0x100;
const CLONE_FS: u64 = 0x200;
const CLONE_FILES: u64 = 0x400;
const CLONE_SIGHAND: u64 = 0x800;
const CLONE_PIDFD: u64 = 0x1000;
const CLONE_VFORK: u64 = 0x4000;
const CLONE_THREAD: u64 = 0x1_0000;
const CLONE_NEWNS: u64 = 0x2_0000;
const CLONE_DETACHED: u64 = 0x40_0000;
const CLONE_NEWCGROUP: u64 = 0x200_0000;
const CLONE_NEWUTS: u64 = 0x400_0000;
const CLONE_NEWIPC: u64 = 0x800_0000;
const CLONE_NEWUSER: u64 = 0x1000_0000;
const CLONE_NEWPID: u64 = 0x2000_0000;
const CLONE_NEWNET: u64 = 0x4000_0000;
/// Every flag that asks for a namespace of its own.
const NEW_NAMESPACE: u64 = CLONE_NEWNS
    | CLONE_NEWCGROUP
    | CLONE_NEWUTS
    | CLONE_NEWIPC
    | CLONE_NEWUSER
    | CLONE_NEWPID
    | CLONE_NEWNET;

/// The flags the legacy clone call reads: its lower 32 bits.
pub fn flags_of(raw: u64) -> u64 {
    raw & 0xffff_ffff
}

/// What copy_process refuses whoever asks, EINVAL: a new mount namespace or
/// user namespace that shares a filesystem view, a thread without shared
/// handlers, shared handlers without shared memory, a thread in a new user
/// or pid namespace, and a pidfd for a thread or with CLONE_DETACHED. Then
/// EPERM for any other namespace, which needs a privilege no guest holds:
/// the namespaces are the personality's, as unshare and setns say too.
pub fn clone_valid(flags: u64) -> Result<(), i64> {
    let has = |f: u64| flags & f != 0;
    let thread = has(CLONE_THREAD);
    if has(CLONE_FS) && (has(CLONE_NEWNS) || has(CLONE_NEWUSER)) {
        return Err(errno::EINVAL);
    }
    if (thread && !has(CLONE_SIGHAND)) || (has(CLONE_SIGHAND) && !has(CLONE_VM)) {
        return Err(errno::EINVAL);
    }
    if thread && has(CLONE_NEWUSER | CLONE_NEWPID) {
        return Err(errno::EINVAL);
    }
    if has(CLONE_PIDFD) && (thread || has(CLONE_DETACHED)) {
        return Err(errno::EINVAL);
    }
    if has(NEW_NAMESPACE) {
        return Err(errno::EPERM);
    }
    Ok(())
}

/// A thread, once clone_valid has passed it: ENOSYS, never a thread that
/// is not what was asked for, when it would keep its own descriptors or
/// working directory (every thread here shares its process's), or would
/// leave its creator parked as CLONE_VFORK asks.
pub fn thread_served(flags: u64) -> Result<(), i64> {
    let shared = CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND | CLONE_THREAD;
    if flags & shared != shared || flags & CLONE_VFORK != 0 {
        return Err(errno::ENOSYS);
    }
    Ok(())
}
