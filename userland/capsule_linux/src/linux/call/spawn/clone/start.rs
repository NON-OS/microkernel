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

//! A checked clone's thread started, counted, and given its mask and its
//! CLEARTID word.

use nonos_libc::{mk_foreign_thread, ForeignFrame};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

const CLONE_CHILD_CLEARTID: u64 = 0x20_0000;

/// The thread `frame` asks for, on `stack` with thread pointer `tls`.
pub(super) fn start(
    guest: &mut Guest,
    frame: &ForeignFrame,
    (stack, tls): (u64, u64),
    flags: u64,
) -> Answer {
    let a = frame.args();
    /* A thread is a task, which RLIMIT_NPROC counts as Linux does. */
    if let Err(e) = super::super::tasks::room(guest.tasks) {
        return Answer::value(errno::fail(e));
    }
    let tid = mk_foreign_thread(guest.pid, frame.rip, stack, tls, frame.pid);
    if tid < 0 {
        return Answer::value(errno::fail(errno::ENOMEM));
    }
    let tid = tid as u32;
    guest.threads.push(tid);
    guest.signals.born(frame.pid, tid); /* its creator's mask, as clone gives */
    /*
     * The SETTID words get the guest's number for the tid, which only the
     * family knows: `serve::clone_tid` writes them with the reply. The
     * CLEARTID word is zeroed and woken when the thread exits: musl's join
     * waits on it.
     */
    if flags & CLONE_CHILD_CLEARTID != 0 {
        guest.clear_tids.push((tid, a[3]));
    }
    Answer::value(errno::ok(u64::from(tid)))
}
