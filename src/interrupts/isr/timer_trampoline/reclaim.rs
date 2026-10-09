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

//! Reclaiming dead processes from the timer tick, and which ticks may.

/// Finish the queued teardowns and free the queued kernel stacks, when the
/// tick interrupted a context that holds no kernel lock.
pub(super) fn on_tick(from_user: bool) {
    /*
     * Never while the interrupted context is a dying one: after exit_and_yield
     * tears the current process down, CURRENT_PID is cleared and the CPU keeps
     * looping on the dead pid's kernel stack under its CR3 until something
     * runnable appears. Draining in that window would free the very stack this
     * trap frame sits on and the live page tables.
     *
     * Nor while kernel code is at work: it holds plain spin locks with
     * interrupts open (init reads the process table once a second), and the
     * teardown takes the same table for writing with interrupts closed, a spin
     * that never ends on the CPU that holds the read.
     *
     * A tick from user mode holds no kernel lock, the rule tick.rs switches
     * by, and neither does the halt a CPU waits in when nothing is runnable:
     * a busy machine reclaims at its next tick in user code, an idle one at
     * its next tick at all.
     */
    let holds_no_lock = from_user || crate::process::accounting::is_idle();
    if holds_no_lock && crate::process::current_pid().is_some() {
        crate::process::exit::drain_pending_teardowns();
        crate::kernel_core::process_spawn::drain_pending_kernel_stacks();
    }
}
