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

//! Putting a process to sleep until a deadline or a wake.

use super::run_queue::remove_from_run_queue;
use super::sleep_table::SLEEPING_PROCESSES;
use super::wake_gen::wake_slot;
use crate::interrupts::disable_interrupts_guard;
use core::sync::atomic::Ordering;

pub fn sleep_until(pid: u32, wake_time_ms: u64) {
    let _irq = disable_interrupts_guard();
    park(pid, wake_time_ms);
}

/// Sleep, unless a wake arrived after `token` was read. The check and the
/// transition happen under one interrupts-off guard, so a wake either lands
/// before (bumping the generation, and this returns without sleeping) or
/// after (finding a genuinely Sleeping process to transition). No gap.
pub fn sleep_until_unless_woken(pid: u32, wake_time_ms: u64, token: u64) {
    let _irq = disable_interrupts_guard();
    if wake_slot(pid).load(Ordering::Acquire) != token {
        return;
    }
    park(pid, wake_time_ms);
}

/*
 * Leave the run queue, then become Sleeping, then publish the deadline. In
 * the old order (deadline, state, queue) a wake landing between the state
 * change and the dequeue left the task Ready but off the queue, and a tick
 * sweep that found the deadline before the state change spent it on a task
 * still Running. Either way the task never ran again.
 */
fn park(pid: u32, wake_time_ms: u64) {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    remove_from_run_queue(pid);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.state.lock() = ProcessState::Sleeping;
    }
    SLEEPING_PROCESSES.write().insert(pid, wake_time_ms);
}
