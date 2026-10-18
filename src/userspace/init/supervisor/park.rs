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

//! How init waits between passes of its loop, and how it moves its own
//! priority.

use crate::process::core::Priority;

const PARK_SLICE_MS: u64 = 20;

// A bare yield left init permanently runnable, so `select_next_process`
// never came up empty and the scheduler's `sti; hlt` idle path was
// unreachable: the vCPU spun at full load with an idle desktop. Sleeping
// on a short deadline takes init off the run queue between passes, which
// lets the CPU actually halt, while still draining the shell's window
// spawn requests inside one compositor frame. Falling back to the yield
// keeps the loop live if init runs before its pid is current.
pub(super) fn park() {
    let Some(pid) = crate::process::current_pid() else {
        crate::sched::yield_now();
        return;
    };
    let wake = crate::time::timestamp_millis().saturating_add(PARK_SLICE_MS);
    crate::sched::sleep_until(pid, wake);
    crate::sched::yield_now();
}

// Set init's priority through the one setter that holds the lock with irqs off.
pub(super) fn set_init_priority(p: Priority) {
    super::super::set_init_priority(p);
}
