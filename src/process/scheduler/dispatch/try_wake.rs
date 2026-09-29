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

/* Waking a sleeper from interrupt context without waiting on its state. */

use super::super::preemption::{note_ready, SCHEDULER_STATS};
use super::run_queue::add_to_run_queue;
use super::sleep_table::SLEEPING_PROCESSES;
use super::wake_gen::wake_slot;
use crate::interrupts::disable_interrupts_guard;
use core::sync::atomic::Ordering;

/*
 * `wake_process` for the timer sweep and the IRQ dispatcher. They run on
 * top of whatever the CPU was doing, and a kernel thread running with
 * interrupts open (init reading a capsule's state to see if it is alive)
 * can be holding the very state lock the wake needs. Spinning on it there
 * never ends: the holder is the code the interrupt stopped, on this CPU.
 * So the lock is only tried. False means the state was busy and nothing
 * changed; the caller keeps the wake pending and retries it later.
 */
pub fn try_wake_process(pid: u32) -> bool {
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
        }
        sleeping
    };
    wake_slot(pid).fetch_add(1, Ordering::AcqRel);
    if woke {
        SLEEPING_PROCESSES.write().remove(&pid);
        add_to_run_queue(pid);
        SCHEDULER_STATS.wakeups.fetch_add(1, Ordering::Relaxed);
        note_ready(pid, &pcb);
    }
    true
}
