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

//! `clone` when it makes a process rather than a thread: a copy, as fork's
//! is, that starts on the stack the caller named, raises the signal it named
//! when it ends, and with CLONE_VFORK parks its parent as vfork does. The tid
//! words it names are written as Linux writes them.

use super::vfork::start;
use super::vfork_flags::{CLONE_CHILD_CLEARTID, CLONE_CHILD_SETTID, CLONE_PARENT_SETTID};
use super::vfork_flags::{CLONE_VFORK, CLONE_VM, CSIGNAL, SERVED};
use crate::linux::abi::errno;
use crate::linux::guest::sigstate::NSIG;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// clone(flags, stack, parent_tid, child_tid, tls) without CLONE_THREAD.
pub fn clone_process(guest: &mut Guest, caller: u32, a: [u64; 6]) -> Answer {
    let (flags, stack) = (a[0], a[1]);
    let why = if flags & !SERVED != 0 {
        Some("clone: flags beyond a copied process")
    } else if flags & CLONE_VM != 0 && flags & CLONE_VFORK == 0 {
        Some("clone: a process sharing its parent's memory")
    } else {
        None
    };
    if let Some(why) = why {
        let line = alloc::format!("[LINUX] unserved {why}, flags {flags:#x}\n");
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
        return Answer::value(errno::fail(errno::ENOSYS));
    }
    let signal = flags & CSIGNAL;
    if signal > NSIG as u64 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    /* The child's own copy of its tid is written before it can read it. */
    let prep = move |child: &mut Guest| {
        let seen = crate::linux::serve::guest_pid(child.pid).to_le_bytes();
        if flags & CLONE_CHILD_SETTID != 0 {
            let _ = child.write(a[3], &seen);
        }
        if flags & CLONE_CHILD_CLEARTID != 0 {
            child.clear_tids.push((child.pid, a[3]));
        }
    };
    let answer = start(guest, caller, stack, signal as u8, flags & CLONE_VFORK != 0, prep);
    if flags & CLONE_PARENT_SETTID != 0 {
        /* Only a child made by this call is in `forked`. */
        if let Some(child) = guest.forked.last().map(|c| c.pid) {
            let seen = crate::linux::serve::guest_pid(child).to_le_bytes();
            let _ = guest.write(a[2], &seen);
        }
    }
    answer
}
