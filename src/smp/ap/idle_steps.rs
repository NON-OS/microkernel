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

//! The two things an idle AP does on each pass: take a process, or halt.

use crate::process::scheduler::dispatch::runnable_process_count;
use crate::process::scheduler::preemption::{clear_reschedule, need_reschedule};
use crate::process::scheduler::selection::{select_next_process, switch_to_process};
use crate::smp::types::CpuDescriptor;
use core::sync::atomic::Ordering;

/// Claim a pid off the shared run queue and switch to it, the way the boot
/// CPU leaves an exiting process. The switch sets this CPU's TSS stack,
/// per-CPU kernel stack, current pid and address space, and does not return:
/// from then on this CPU schedules the way the boot CPU does, by preemption
/// and yield from inside whatever it is running. This used to enter
/// `sched::schedule`, which runs only the kernel task queues, so an AP never
/// ran a single user process.
///
/// True when the arch layer refused the pid, which it has already marked so
/// it is not picked again; false when there was nothing to take.
pub(super) fn take_work(cpu: &CpuDescriptor) -> bool {
    if !need_reschedule() && runnable_process_count() == 0 {
        return false;
    }
    clear_reschedule();
    cpu.set_stage(crate::smp::Stage::EnteringScheduler);
    let Some(next) = select_next_process() else {
        return false;
    };
    cpu.idle.store(false, Ordering::SeqCst);
    switch_to_process(next);
    true
}

/// Halt until an interrupt, counted as idle time.
pub(super) fn halt_once(cpu: &CpuDescriptor) {
    cpu.set_stage(crate::smp::Stage::IdleLoop);
    crate::process::accounting::idle_enter();
    /*
     * Opens interrupts for the halt and masks them again after it.
     */
    crate::arch::idle::wait_for_interrupt();
    crate::process::accounting::idle_leave();
    cpu.idle.store(false, Ordering::Relaxed);
    cpu.idle_cycles.fetch_add(1, Ordering::Relaxed);
}
