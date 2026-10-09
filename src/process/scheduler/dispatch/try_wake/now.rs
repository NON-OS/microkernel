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

//! The wake an interrupt makes on the spot, or leaves for the next tick.

use super::super::super::preemption::{note_ready, SCHEDULER_STATS};
use super::super::run_queue::add_to_run_queue;
use super::super::sleep_table::SLEEPING_PROCESSES;
use super::super::wake_gen::wake_slot;
use super::deferred::defer;
use crate::interrupts::disable_interrupts_guard;
use core::sync::atomic::Ordering;

/*
 * `wake_process` for the timer sweep and the IRQ dispatcher. They run on
 * top of whatever the CPU was doing, and a kernel thread running with
 * interrupts open (init reading a capsule's state to see if it is alive)
 * can be holding the very state lock the wake needs. Spinning on it there
 * never ends: the holder is the code the interrupt stopped, on this CPU.
 * So the locks are only tried; a busy one leaves the wake for the next tick
 * of any CPU. False means even that list was full, and the caller keeps
 * the wake pending and retries it later.
 */
pub fn try_wake_process(pid: u32) -> bool {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let _irq = disable_interrupts_guard();
    // Bumped before the state lock, as `wake_process` does: a sleeper checks
    // its token under that lock, so it sees this bump or is Sleeping by then.
    // A bump for a failed try only makes a sleeper look again.
    wake_slot(pid).fetch_add(1, Ordering::AcqRel);
    let pcb = match PROCESS_TABLE.try_find_by_pid(pid) {
        // The table is being changed by a spawn or an exit on another CPU.
        // Dropped here, with no count bumped, the wake left a driver asleep
        // until its wait ran out, 100 ms per interrupt by default, and boot
        // is when drivers wait on interrupts while capsules spawn.
        None => return defer(pid),
        Some(None) => return true,
        Some(Some(pcb)) => pcb,
    };
    let woke = {
        let Some(mut state) = pcb.state.try_lock() else {
            // Another CPU holds the state for a moment: an interrupt cannot
            // spin on it, and a wake dropped here left the driver asleep until
            // its deadline. It is kept and done on the next tick of any CPU.
            return defer(pid);
        };
        let sleeping = *state == ProcessState::Sleeping;
        if sleeping {
            *state = ProcessState::Ready;
            /* Under the state lock, as in `wake_process`. */
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
