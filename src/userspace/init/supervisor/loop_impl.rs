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

//! Init's residual loop after every capsule has been spawned.

use crate::process::core::Priority;

const TICK_INTERVAL_MS: u64 = 1000;
const PARK_SLICE_MS: u64 = 20;

pub(crate) fn init_loop() -> ! {
    // Tell the queue side which process drains it.
    if let Some(pid) = crate::process::current_pid() {
        crate::userspace::init::owns_the_queues(pid);
    }
    let mut last_tick = 0u64;
    #[cfg(feature = "microkernel-setup-wizard")]
    let mut desktop_started = false;
    loop {
        let now = crate::time::timestamp_millis();
        if now >= last_tick + TICK_INTERVAL_MS {
            crate::services::lifecycle::tick();
            last_tick = now;
        }
        #[cfg(feature = "microkernel-setup-wizard")]
        if !desktop_started && !crate::userspace::capsule_setup_wizard::shared_state().is_alive() {
            super::super::spawn_plan::spawn_post_wizard();
            desktop_started = true;
        }
        // Perform any window-instance spawns the shell requested.
        crate::userspace::init::service_instance_spawns();
        crate::userspace::init::service_installs();
        // Back to Low now the queues are empty. Raising is the
        // producer's job; only this loop can know when to stop.
        if !crate::userspace::init::instance_spawns_pending() {
            crate::userspace::init::settle_priority();
        }
        park();
    }
}

// A bare yield left init permanently runnable, so `select_next_process` never
// came up empty and the scheduler's `sti; hlt` idle path was unreachable: the
// vCPU spun at full load with an idle desktop.
fn park() {
    let Some(pid) = crate::process::current_pid() else {
        crate::sched::yield_now();
        return;
    };
    let wake = crate::time::timestamp_millis().saturating_add(PARK_SLICE_MS);
    crate::sched::sleep_until(pid, wake);
    crate::sched::yield_now();
}

// Set init's own scheduling priority.
fn set_init_priority(p: Priority) {
    use crate::process::core::{CURRENT_PID, PROCESS_TABLE};
    use core::sync::atomic::Ordering;
    let pid = CURRENT_PID.load(Ordering::Relaxed);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.priority.lock() = p;
    }
}
