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

//! clone's checks: what Linux refuses, and what this personality cannot make.

use nonos_libc::ForeignFrame;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::super::clone_flags::{clone_valid, flags_of, thread_served};
use super::start::start;

const CLONE_SETTLS: u64 = 0x80000;

/// A Linux clone child resumes at the instruction after its parent's
/// `syscall`, on its parent's registers with rax zero and rsp the new stack.
/// Both runtimes that start threads here call through a register in the
/// child: musl's `__clone` pops the argument and calls r9, Go's calls r12.
pub fn clone(guest: &mut Guest, frame: &ForeignFrame) -> Answer {
    let a = frame.args();
    let (flags, stack) = (flags_of(a[0]), a[1]);
    /*
     * The fifth argument is a thread pointer only when the flag says so;
     * without it the child keeps its parent's.
     */
    let tls = if flags & CLONE_SETTLS != 0 { a[4] } else { 0 };
    /*
     * What Linux refuses outright is EINVAL or EPERM here too; a thread this
     * personality cannot make as asked, one keeping its own descriptors or
     * working directory, is ENOSYS rather than a thread that shares them.
     */
    if let Err(e) = clone_valid(flags).and_then(|()| thread_served(flags)) {
        if e == errno::ENOSYS {
            let line = alloc::format!("[LINUX] unserved clone: a thread with flags {flags:#x}\n");
            let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
        }
        return Answer::value(errno::fail(e));
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
    start(guest, frame, (stack, tls), flags)
}
