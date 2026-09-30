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

//! Where a CPU waits once the process it ran is dead.
//!
//! The CPU is still on that process's kernel stack and has nowhere to
//! return to, so it takes the next runnable process and never comes back.
//! Until there is one it halts where it stands. The stack stays in use until
//! then, and the teardown queues hold it back (see `pending::drain`); the
//! page tables are left first, so the process can be reaped meanwhile.

use core::sync::atomic::Ordering;

use crate::process::core::{clear_current_if, Pid};
use crate::process::scheduler::selection::{
    leave_address_space, select_next_process, switch_to_process,
};

/// Leave `pid`, dead already, for whatever runs next. Also the path of a
/// process killed from another CPU once the scheduler finds it here.
pub(crate) fn park(pid: Pid) -> ! {
    clear_current_if(pid);
    leave_address_space();
    loop {
        // Published before the queue is read, so a wake sent after the
        // read finds it, stays pending while interrupts are masked, and
        // ends the halt at once.
        mark_idle(true);
        if let Some(next) = select_next_process() {
            mark_idle(false);
            switch_to_process(next);
            // Refused: this CPU waits after all.
            mark_idle(true);
        }
        crate::process::accounting::idle_enter();
        crate::arch::idle_cpu();
        crate::process::accounting::idle_leave();
        mark_idle(false);
    }
}

// A CPU that makes a process runnable wakes only a CPU marked idle. The
// single-CPU image has no one to send the wake and keeps its old behaviour.
fn mark_idle(idle: bool) {
    if cfg!(feature = "nonos-smp") {
        crate::smp::current_cpu().idle.store(idle, Ordering::SeqCst);
    }
}
