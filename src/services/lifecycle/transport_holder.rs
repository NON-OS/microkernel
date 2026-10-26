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

//! Who holds a transport lock, asked when one cannot be taken (L77e): a
//! holder that can never run again gives it up to the next caller, and a pid
//! handed to a new process gives back any lock its old holder still had.

use spin::Mutex;

use super::transport_lock::HOLDERS;

/// Killed, or gone from the table, and on no CPU's tables.
pub(crate) fn never_runs_again(pid: u32) -> bool {
    use crate::process::core::{ProcessState, PROCESS_TABLE};
    let dead =
        |s: &ProcessState| matches!(s, ProcessState::Zombie(_) | ProcessState::Terminated(_));
    let ended = PROCESS_TABLE.find_by_pid(pid).is_none_or(|p| dead(&p.state.lock()));
    let on_cpu = crate::process::scheduler::selection::cpu_on_tables(pid).is_some();
    super::transport_rule::takes_over(pid, ended, on_cpu)
}

/// A pid given to a new process: a lock its old holder still has is taken
/// back now, before the new process could be mistaken for that holder.
pub(crate) fn forget(pid: u32) {
    let _quiet = super::rest_mark::quiet();
    let mut holders = HOLDERS.lock();
    for &(key, _) in holders.iter().filter(|h| h.1 == pid) {
        // SAFETY: every key is a `&'static Mutex<()>` given to `lock_yielding`.
        let lock = unsafe { &*(key as *const Mutex<()>) };
        if lock.is_locked() {
            // SAFETY: held, its row names the old holder, which is finalized.
            unsafe { lock.force_unlock() };
        }
    }
    holders.retain(|h| h.1 != pid);
}
