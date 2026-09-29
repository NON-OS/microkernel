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

//! The tick's sweep of sleepers whose deadline has passed.

use super::sleep_table::SLEEPING_PROCESSES;
use super::wake::wake_process;
use crate::interrupts::disable_interrupts_guard;

pub fn check_sleeping_processes() {
    let _irq = disable_interrupts_guard();
    let current_time_ms = crate::time::timestamp_millis();
    let mut pids_to_wake = [0u32; 64];
    let mut count = 0usize;
    {
        let sleeping = SLEEPING_PROCESSES.read();
        for (&pid, &wt) in sleeping.iter() {
            if current_time_ms >= wt && count < pids_to_wake.len() {
                pids_to_wake[count] = pid;
                count += 1;
            }
        }
    }
    for &pid in &pids_to_wake[..count] {
        /*
         * The deadline has passed, so the entry is spent regardless of
         * whether the wake transitions the process (it may already be
         * Running via an early-return path); leaving a stale entry behind
         * would make this sweep re-chew it every tick until the 64-slot
         * budget is exhausted.
         */
        SLEEPING_PROCESSES.write().remove(&pid);
        wake_process(pid);
    }
}
