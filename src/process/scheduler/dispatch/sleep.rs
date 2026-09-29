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
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let _irq = disable_interrupts_guard();
    SLEEPING_PROCESSES.write().insert(pid, wake_time_ms);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.state.lock() = ProcessState::Sleeping;
    }
    remove_from_run_queue(pid);
}

/// Sleep, unless a wake arrived after `token` was read. The check and the
/// transition happen under one interrupts-off guard, so a wake either lands
/// before (bumping the generation, and this returns without sleeping) or
/// after (finding a genuinely Sleeping process to transition). No gap.
pub fn sleep_until_unless_woken(pid: u32, wake_time_ms: u64, token: u64) {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let _irq = disable_interrupts_guard();
    if wake_slot(pid).load(Ordering::Acquire) != token {
        return;
    }
    SLEEPING_PROCESSES.write().insert(pid, wake_time_ms);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.state.lock() = ProcessState::Sleeping;
    }
    remove_from_run_queue(pid);
}
