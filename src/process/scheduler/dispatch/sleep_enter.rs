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

use super::run_queue::remove_from_run_queue;
use super::sleep_table::SLEEPING_PROCESSES;
use super::wake_gen::wake_slot;
use crate::interrupts::disable_interrupts_guard;
use core::sync::atomic::Ordering;

/*
 * The state and the deadline change together, under the state lock, and a
 * wake strips the deadline under that same lock. Interrupts off alone made
 * this atomic only on one CPU. With the tick sweep and the wakers running on
 * other CPUs, two orders stranded a sleeper for good: the sweep took a
 * deadline that had just been inserted, found the process still Running and
 * woke nothing, and the process then went Sleeping with no deadline left; or
 * a waker transitioned the process, dropped the lock, the process ran and
 * slept again, and the waker's late remove took the new deadline. Either way
 * the process slept until an explicit wake that for a timed wait never
 * comes: the input router parked like this in a 1 ms receive and drained no
 * input again.
 *
 * The run queue is left first, under the same lock: a wake landing between
 * the state change and the dequeue left the task Ready but off the queue,
 * and it never ran again.
 *
 * Lock order is state lock, then the run queue and the sleep table; the
 * sweep never holds the table while it takes a state lock. The insert may
 * allocate, which is safe here: the heap lock is a leaf taken with
 * interrupts off.
 */
pub(super) fn enter_sleep(pid: u32, wake_time_ms: u64, token: Option<u64>) {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let _irq = disable_interrupts_guard();
    let pcb = PROCESS_TABLE.find_by_pid(pid);
    let mut state = pcb.as_ref().map(|p| p.state.lock());
    if token.is_some_and(|t| wake_slot(pid).load(Ordering::Acquire) != t) {
        return;
    }
    remove_from_run_queue(pid);
    if let Some(state) = state.as_mut() {
        **state = ProcessState::Sleeping;
    }
    SLEEPING_PROCESSES.write().insert(pid, wake_time_ms);
}
