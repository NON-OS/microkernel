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

//! The copy every new process starts as: fork's, vfork's and a clone that
//! makes a process all come here.

use nonos_libc::{mk_foreign_fork_at, mk_foreign_resume};

use super::fork_copy::copy_spans;
use super::tasks::room;
use crate::linux::abi::errno;
use crate::linux::guest::sigstate::SIGCHLD;
use crate::linux::guest::Guest;

/// A new process copied from this one, running, and adopted by the family
/// once this answer is given. It raises `exit_signal` at its parent when it
/// ends. Its signal state is the forking thread's, as Linux's fork gives it.
/// It starts on `stack` when that is not zero, as a clone naming a stack asks.
/// `prep` runs on the child before it runs at all.
pub(super) fn fork_child(
    guest: &mut Guest,
    caller: u32,
    stack: u64,
    exit_signal: u8,
    prep: impl FnOnce(&mut Guest),
) -> Result<u32, u64> {
    /* RLIMIT_NPROC: the family's tasks, and any made earlier in this answer. */
    room(guest.tasks.saturating_add(guest.forked.len())).map_err(errno::fail)?;
    let child = mk_foreign_fork_at(caller, stack);
    if child < 0 {
        return Err(errno::fail(errno::ENOMEM));
    }
    let child = child as u32;
    if !copy_spans(guest, child) {
        return Err(errno::fail(errno::ENOMEM));
    }
    /*
     * The thread pointer is a register, not memory, so copying the spans does
     * not carry it. The kernel fork carries the forking thread's own FS to the
     * child, which is right whichever thread forked; the personality's single
     * fs_base is only the last thread to set one and would be wrong here.
     */
    /*
     * The child's state goes to the serve loop before the child runs, so its
     * first trap finds a guest that owns it.
     */
    let mut state = guest.fork_state(child);
    state.signals = guest.signals.forked(caller, child);
    state.signals.exit_signal = exit_signal;
    prep(&mut state);
    guest.forked.push(state);
    if mk_foreign_resume(child) < 0 {
        guest.forked.pop();
        return Err(errno::fail(errno::ENOMEM));
    }
    guest.children.push(child);
    if exit_signal != SIGCHLD {
        guest.signals.clone_kids.push(child);
    }
    Ok(child)
}
