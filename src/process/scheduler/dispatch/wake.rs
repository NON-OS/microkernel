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

//! Waking a sleeper, by event or because its deadline passed.

use super::super::preemption::{note_ready, SCHEDULER_STATS};
use super::run_queue::add_to_run_queue;
use super::sleep_table::SLEEPING_PROCESSES;
use super::wake_gen::wake_slot;
use crate::interrupts::disable_interrupts_guard;
use core::sync::atomic::Ordering;

pub fn wake_process(pid: u32) {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let _irq = disable_interrupts_guard();
    wake_slot(pid).fetch_add(1, Ordering::AcqRel);
    let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) else {
        return;
    };
    let woke = {
        let mut state = pcb.state.lock();
        let sleeping = *state == ProcessState::Sleeping;
        if sleeping {
            *state = ProcessState::Ready;
            /*
             * Only a wake that actually transitioned the process may strip
             * its sleep deadline: a wake landing on a Running/Ready target
             * must not destroy the timeout of a sleep the target is about to
             * enter (or re-enter), or that sleep becomes unwakeable by the
             * tick sweep. The generation bump above is what tells that
             * target the wake happened. The strip happens under the state
             * lock so it can only ever take the deadline of the sleep it
             * ended, never one the process entered after it.
             */
            SLEEPING_PROCESSES.write().remove(&pid);
        }
        sleeping
    };
    if woke {
        add_to_run_queue(pid);
        SCHEDULER_STATS.wakeups.fetch_add(1, Ordering::Relaxed);
        note_ready(pid, &pcb);
    }
}
