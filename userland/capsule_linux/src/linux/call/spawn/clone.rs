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

//! `clone`, for threads only.

use nonos_libc::{mk_foreign_thread, ForeignFrame};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

const CLONE_VM: u64 = 0x100;
const CLONE_THREAD: u64 = 0x10000;
const CLONE_SETTLS: u64 = 0x80000;
const CLONE_PARENT_SETTID: u64 = 0x10_0000;
const CLONE_CHILD_CLEARTID: u64 = 0x20_0000;
const CLONE_CHILD_SETTID: u64 = 0x100_0000;

/// A Linux clone child resumes at the instruction after its parent's
/// `syscall`, on its parent's registers with rax zero and rsp the new stack.
/// Both runtimes that start threads here call through a register in the
/// child: musl's `__clone` pops the argument and calls r9, Go's calls r12.
pub fn clone(guest: &mut Guest, frame: &ForeignFrame) -> Answer {
    let a = frame.args();
    let (flags, stack) = (a[0], a[1]);
    // The fifth argument is a thread pointer only when the flag says so;
    // without it the child keeps its parent's.
    let tls = if flags & CLONE_SETTLS != 0 { a[4] } else { 0 };
    if flags & (CLONE_VM | CLONE_THREAD) != CLONE_VM | CLONE_THREAD {
        /*
         * A new process, not a thread. That is fork, and fork needs an
         * address space copy no peer call offers.
         */
        return Answer::value(errno::fail(errno::ENOSYS));
    }
    if frame.rip == 0 {
        /*
         * The kernel is not yet passing the guest's return address, so there
         * is nowhere correct to start the child.
         */
        return Answer::value(errno::fail(errno::ENOSYS));
    }
    if stack == 0 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    let tid = mk_foreign_thread(guest.pid, frame.rip, stack, tls, frame.pid);
    if tid < 0 {
        return Answer::value(errno::fail(errno::ENOMEM));
    }
    let tid = tid as u32;
    guest.threads.push(tid);
    // Linux writes the new tid where the caller asked, and ignores a word it
    // cannot write; musl keeps the parent's copy as the thread's own tid.
    if flags & CLONE_PARENT_SETTID != 0 {
        let _ = guest.write(a[2], &tid.to_le_bytes());
    }
    if flags & CLONE_CHILD_SETTID != 0 {
        let _ = guest.write(a[3], &tid.to_le_bytes());
    }
    // Zeroed and woken when the thread exits: musl's join waits on it.
    if flags & CLONE_CHILD_CLEARTID != 0 {
        guest.clear_tids.push((tid, a[3]));
    }
    Answer::value(errno::ok(u64::from(tid)))
}
