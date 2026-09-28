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

//! Stopping a marked guest thread at a timer tick that interrupted it.

use super::marks::{any, take};
use super::tick_frame::{to_user, to_words, WORDS};
use crate::process::foreign::frame::{ForeignFrame, NR_INTERRUPTED};
use crate::process::foreign::trap_table::{park, Answer};
use crate::process::foreign::{registry, signal_fpu, trap_frame, trap_wait};

/// Called by the timer trampoline, after the tick, for a tick that
/// interrupted user mode. `frame` is the interrupted register file the
/// trampoline restores (`tick_frame`).
pub fn on_user_tick(frame: &mut [u64; WORDS]) {
    if !any() || !crate::smp::preempt_enabled() {
        return;
    }
    let Some(pid) = crate::process::current_pid() else {
        return;
    };
    if !take(pid) {
        return;
    }
    let Some(supervisor) = registry::supervisor_of(pid) else {
        return;
    };
    let fs_base = crate::process::with_process(pid, |p| p.get_tls_base()).unwrap_or(0);
    let held = to_user(frame, fs_base);
    trap_frame::keep(pid, held);
    if !park(ForeignFrame::new(pid, NR_INTERRUPTED, [0; 6], held.rip)) {
        trap_frame::drop_frame(pid);
        return;
    }
    crate::sched::wake_process(supervisor);
    if let Answer::Deliver(to) = trap_wait::wait_raw(pid) {
        signal_fpu::enter_fpu(pid);
        crate::arch::context::set_user_tls(to.fs_base);
        *frame = to_words(&to);
    }
}
