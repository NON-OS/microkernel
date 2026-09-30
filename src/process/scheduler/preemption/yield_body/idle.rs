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
// caller's recv/wait loop spinning at CPL=0 with IF=0 (SFMASK) —
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
    mark_idle(true);
    crate::arch::idle::wait_for_interrupt();
    mark_idle(false);
    crate::process::accounting::idle_leave();
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
