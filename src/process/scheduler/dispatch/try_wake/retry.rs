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

//! A deferred wake made again from a tick, on the same terms as the first try.

use super::super::super::preemption::{note_ready, SCHEDULER_STATS};
use super::super::run_queue::add_to_run_queue;
use super::super::sleep_table::SLEEPING_PROCESSES;
use crate::interrupts::disable_interrupts_guard;
use core::sync::atomic::Ordering;

pub(super) fn try_wake_now(pid: u32) -> bool {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let _irq = disable_interrupts_guard();
    let pcb = match PROCESS_TABLE.try_find_by_pid(pid) {
        None => return false,
        Some(None) => return true,
        Some(Some(pcb)) => pcb,
    };
    let woke = {
        let Some(mut state) = pcb.state.try_lock() else {
            return false;
        };
        let sleeping = *state == ProcessState::Sleeping;
        if sleeping {
            *state = ProcessState::Ready;
            SLEEPING_PROCESSES.write().remove(&pid);
        }
        sleeping
    };
    if woke {
        add_to_run_queue(pid);
        SCHEDULER_STATS.wakeups.fetch_add(1, Ordering::Relaxed);
        note_ready(pid, &pcb);
    }
    true
}
