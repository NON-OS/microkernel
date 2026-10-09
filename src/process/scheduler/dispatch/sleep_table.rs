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

//! The sleeping processes and the deadlines they wake at.

use crate::interrupts::disable_interrupts_guard;
use alloc::collections::BTreeMap;

/*
 * The timer tick sweeps this table (check_sleeping_processes) to wake expired
 * sleepers, so it is reached from interrupt context. Every normal-path access
 * holds interrupts off while it has the lock, or a tick landing mid-update
 * would spin on a lock the interrupted code cannot release. The sweep itself
 * already runs with interrupts off, so its guards are simply no-ops.
 */
pub(super) static SLEEPING_PROCESSES: spin::RwLock<BTreeMap<u32, u64>> =
    spin::RwLock::new(BTreeMap::new());

pub fn is_sleeping(pid: u32) -> bool {
    let _irq = disable_interrupts_guard();
    SLEEPING_PROCESSES.read().contains_key(&pid)
}

pub fn get_remaining_sleep(pid: u32) -> Option<u64> {
    let _irq = disable_interrupts_guard();
    let sleeping = SLEEPING_PROCESSES.read();
    if let Some(&wake_time) = sleeping.get(&pid) {
        let now = crate::time::timestamp_millis();
        if wake_time > now {
            Some(wake_time - now)
        } else {
            Some(0)
        }
    } else {
        None
    }
}
