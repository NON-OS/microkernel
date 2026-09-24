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

//! A refused syscall, parked until its supervisor answers.
//!
//! The guest's thread sleeps inside the syscall it made, so from the
//! guest's side nothing happened except that its `syscall` took a while.
//! The supervisor is a separate process with its own capabilities; the
//! only thing crossing between them is a register frame out and one value
//! back.

use super::frame::ForeignFrame;
use super::registry;
use super::trap_table::{park, take_answer};

/// The kernel's answer to a syscall number it does not know, made by the
/// supervisor rather than by the kernel. `None` when the caller has no
/// supervisor, which leaves the refusal the entry shim would have given.
pub fn redirect(nr: u64, args: [u64; 6], rip: u64) -> Option<u64> {
    let pid = crate::process::current_pid()?;
    let supervisor = registry::supervisor_of(pid)?;
    park(ForeignFrame::new(pid, nr, args, rip));
    crate::sched::wake_process(supervisor);
    Some(wait_for_answer(pid))
}

/*
 * The guest holds no locks here and owns nothing the supervisor needs, so
 * a supervisor that never answers costs exactly one parked thread. The
 * answer is re-checked after taking the wake token and again after the
 * sleep, so a reply landing in either window is not slept through.
 */
fn wait_for_answer(pid: u32) -> u64 {
    loop {
        if let Some(value) = take_answer(pid) {
            return value;
        }
        let token = crate::sched::wake_token(pid);
        if let Some(value) = take_answer(pid) {
            return value;
        }
        crate::sched::sleep_until_unless_woken(pid, u64::MAX, token);
        crate::sched::yield_now();
    }
}
