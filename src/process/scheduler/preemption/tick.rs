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

use super::super::realtime;
use super::proc_ticks;
use super::state::{set_reschedule, spend_time_slice, SCHEDULER_STATS};
use core::sync::atomic::Ordering;

pub fn tick() {
    /*
     * A tick is taken with interrupts open, which no switch path allows, so
     * any switch this CPU started has finished: it is off the stack it left.
     * The first process is started by hand rather than switched to, and is
     * adopted here the first time it is interrupted.
     */
    super::super::selection::release_leaving();
    super::super::selection::adopt_current(crate::process::CURRENT_PID.load(Ordering::Relaxed));
    // A halted processor belongs to nobody: the tick that wakes it is idle
    // time, not the last process's.
    if crate::process::accounting::is_idle() {
        crate::process::accounting::tick_idle();
    } else {
        let pid = crate::process::CURRENT_PID.load(Ordering::Relaxed);
        proc_ticks::charge_tick(pid);
        crate::process::accounting::tick_charge(pid);
    }
    SCHEDULER_STATS.tick_count.fetch_add(1, Ordering::SeqCst);
    // This CPU's own slice. The tick that takes it from one to zero is the one
    // that exhausted it.
    if spend_time_slice() == 1 {
        SCHEDULER_STATS.time_slice_exhaustions.fetch_add(1, Ordering::SeqCst);
        if crate::sys::policy::kernel_preempt() {
            set_reschedule();
        }
    }
    if realtime::has_realtime_tasks() {
        set_reschedule();
    }
    /*
     * Killed from another CPU while it ran here: switch away at this tick
     * rather than resume it. Asked only of a tick that interrupted user mode,
     * the only kind that switches, when this CPU holds no kernel lock.
     */
    let pid = crate::process::CURRENT_PID.load(Ordering::Relaxed);
    if pid != 0
        && crate::smp::percpu::current().tick_from_user.load(Ordering::Relaxed)
        && super::super::selection::is_dead(pid)
    {
        set_reschedule();
    }
}
