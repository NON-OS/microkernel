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

use super::frame::ForeignFrame;
use super::frame_snapshot::{capture, FRAME_WORDS};
use super::registry;
use super::trap_table::park;
use super::trap_wait::wait_for_answer;
use crate::process::core::ProcessState;

/// The kernel's answer to a syscall number it does not know, made by the
/// supervisor rather than by the kernel.
pub fn redirect(nr: u64, args: [u64; 6], frame: &[u64; FRAME_WORDS]) -> Option<u64> {
    /*
     * The numbers this kernel hands a supervisor for its own reasons, a death
     * and a tick stop, are no syscall's: a guest naming one gets ENOSYS, as
     * for any number nobody serves, not a stop its supervisor answers 0.
     */
    if nr >= super::frame::NR_INTERRUPTED {
        return None;
    }
    let pid = crate::process::current_pid()?;
    let Some(supervisor) = registry::supervisor_of(pid) else {
        end_if_dead(pid);
        return None;
    };
    super::guest_stats::called(pid);
    /*
     * The frame is reachable only while this call is on the stack, and a fork
     * asks for it long afterwards, so it is copied aside now.
     */
    let saved = capture(frame, super::frame_cpu::user_rsp());
    super::trap_frame::keep(pid, saved);
    if !park(ForeignFrame::new(pid, nr, args, saved.rip)) {
        end_if_dead(pid);
        return Some(super::trap_reply::ABANDONED);
    }
    crate::sched::wake_process(supervisor);
    let value = wait_for_answer(pid);
    end_if_dead(pid);
    Some(value)
}

/*
 * A guest is ended with its supervisor, and on several CPUs that can happen
 * while it runs on another one: it loses its row there, and its supervisor
 * cannot answer. Returning to it would give it an error for this call and
 * for every one after, so it never gets back to user mode; this CPU leaves
 * it the way an exiting process leaves itself.
 */
fn end_if_dead(pid: u32) {
    let dead = crate::process::with_process(pid, |pcb| {
        matches!(*pcb.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
    });
    if dead == Some(true) {
        crate::process::scheduler::selection::adopt_current(pid);
        crate::process::exit::park_dead(pid);
    }
}
