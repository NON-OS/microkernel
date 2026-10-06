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

// Nothing is runnable: every process is parked on a timeout or an
// IRQ/IPC wake. Yield used to plain-return here, which sent the
// caller's recv/wait loop spinning at CPL=0 with IF=0 (SFMASK):
// the timer could never fire, so time froze and no sleeper could
// ever wake. Waiting with interrupts open is what unfroze it, and the
// handler (timer tick or broker IRQ) refills the run queue before
// control returns. How a given CPU waits without losing an already
// pending wake is the arch layer's business.
pub(super) fn idle_until_interrupt() {
    // The halt belongs to no process: without the mark, the tick that ends
    // it was charged to whichever process yielded last, and an idle desktop
    // read as a third of the processor spent in init.
    crate::process::accounting::idle_enter();
    crate::arch::idle::wait_for_interrupt();
    mark_idle(false);
    crate::process::accounting::idle_leave();
}

/*
 * The pick, made with this CPU already marked idle, as `park` and the AP idle
 * loop make it. Marked only after a pick that found nothing, a CPU that
 * queued work in between saw no idle mark, sent no wake, and the halt that
 * followed lasted to the next 10 ms tick. Marked first, that wake is sent and
 * stays pending with interrupts masked, so the halt ends at once.
 */
pub(super) fn select_marked_idle() -> Option<u32> {
    mark_idle(true);
    let next = super::super::super::selection::select_next_process();
    if next.is_some() {
        pass_on_spent_wake();
    }
    next
}

/*
 * A wake sent while this CPU was marked may be for work it is not taking, so
 * it goes on to another idle CPU rather than leave that work to a tick. The
 * queue holds running pids too: at worst the CPU woken finds nothing.
 */
fn pass_on_spent_wake() {
    if !cfg!(feature = "nonos-smp") {
        return;
    }
    let spent = !crate::smp::current_cpu().idle.swap(false, core::sync::atomic::Ordering::SeqCst);
    if spent && crate::process::scheduler::dispatch::runnable_process_count() > 0 {
        crate::smp::wake_idle_cpu();
    }
}

/*
 * A CPU that makes a process runnable sends a wake only to a CPU marked idle.
 * Unmarked, a CPU waiting here heard of new work at its next tick at best.
 * The single-CPU image has no one to send it and keeps its old behaviour.
 */
fn mark_idle(idle: bool) {
    if cfg!(feature = "nonos-smp") {
        crate::smp::current_cpu().idle.store(idle, core::sync::atomic::Ordering::SeqCst);
    }
}
