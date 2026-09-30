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

use crate::process::scheduler::dispatch::runnable_process_count;
use crate::process::scheduler::preemption::{clear_reschedule, need_reschedule};
use crate::process::scheduler::selection::{select_next_process, switch_to_process};
use crate::smp::state::CPU_DESCRIPTORS;
use core::sync::atomic::Ordering;

/// Where an AP lives when it has nothing to run, and where it takes its
/// first process from.
///
/// It takes one the way the boot CPU leaves an exiting process: claim a pid
/// off the shared run queue and switch to it, which sets this CPU's TSS stack,
/// per-CPU kernel stack, current pid and address space. That switch does not
/// return, and from then on this CPU schedules the way the boot CPU does, by
/// preemption and yield from inside whatever it is running. This used to
/// enter `sched::schedule`, which runs only the kernel task queues, so an AP
/// never ran a single user process.
///
/// The halt decision is made against the run queue itself, not the reschedule
/// flag alone. A CPU that enqueues work may not know this one is idle and may
/// skip the IPI; if the flag were the only signal, that task would sit
/// unclaimed until some unrelated interrupt happened to land here. The IPI is
/// a latency optimisation, not the correctness condition.
pub(super) fn ap_idle_loop(cpu_id: u32) -> ! {
    let cpu = &CPU_DESCRIPTORS[cpu_id as usize];
    cpu.set_stage(crate::smp::Stage::IdleLoop);
    let runs_user = super::user_setup::is_ready(cpu_id);
    loop {
        // Interrupts off across the test, so work appearing between the test
        // and the halt cannot be missed.
        // SAFETY: eK@nonos.systems - masking interrupts on this CPU only. The
        // window is closed again by the wait below on every path.
        unsafe {
            core::arch::asm!("cli", options(nostack, nomem));
        }

        // Idle is published before the queue is read. A CPU that enqueues
        // after the read then sees the mark and sends the wake, which stays
        // pending across the masked window and ends the halt at once. A CPU
        // that may not run user code never claims to be idle: the wake goes
        // to one idle CPU only, and it would be spent on this one.
        cpu.idle.store(runs_user, Ordering::SeqCst);

        // Set before the queue is consulted. Consulting it takes a lock with
        // interrupts already masked, which is a place a CPU can stop without
        // anything outside it being able to tell.
        cpu.set_stage(crate::smp::Stage::IdleCheckingQueue);

        if runs_user && (need_reschedule() || runnable_process_count() > 0) {
            clear_reschedule();
            cpu.set_stage(crate::smp::Stage::EnteringScheduler);
            if let Some(next) = select_next_process() {
                cpu.idle.store(false, Ordering::SeqCst);
                // Returns only when the arch layer refused the pid, which it
                // has already marked so it is not picked again.
                switch_to_process(next);
                continue;
            }
        }

        cpu.set_stage(crate::smp::Stage::IdleLoop);
        crate::process::accounting::idle_enter();
        // Opens interrupts for the halt and masks them again after it.
        crate::arch::idle::wait_for_interrupt();
        crate::process::accounting::idle_leave();

        cpu.idle.store(false, Ordering::Relaxed);
        cpu.idle_cycles.fetch_add(1, Ordering::Relaxed);
    }
}
