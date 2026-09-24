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

//! Raising init when work is queued for it.

use core::sync::atomic::{AtomicU32, Ordering};

use alloc::sync::Arc;

use crate::process::core::{Priority, ProcessControlBlock, PROCESS_TABLE};

/// Recorded by init itself rather than assumed.
static INIT_PID: AtomicU32 = AtomicU32::new(0);

/// Called once, by init, before it starts draining.
pub(crate) fn owns_the_queues(pid: u32) {
    INIT_PID.store(pid, Ordering::Release);
}

fn init_pcb() -> Option<Arc<ProcessControlBlock>> {
    match INIT_PID.load(Ordering::Acquire) {
        0 => None,
        pid => PROCESS_TABLE.find_by_pid(pid),
    }
}

/// Promote init so the work just queued is drained promptly, and wake it
/// in case it is asleep between passes.
pub(crate) fn nudge() {
    let pid = INIT_PID.load(Ordering::Acquire);
    if let Some(pcb) = init_pcb() {
        *pcb.priority.lock() = Priority::Normal;
        crate::sched::wake_process(pid);
    }
}

/// Drop back once the queues are empty. Called by init, the only place
/// that knows there is nothing left to do.
pub(crate) fn settle() {
    if let Some(pcb) = init_pcb() {
        *pcb.priority.lock() = Priority::Low;
    }
}
