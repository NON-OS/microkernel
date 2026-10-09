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

use super::super::super::selection::{adopt_current, is_dead, release_leaving, switch_to_process};
use super::super::hand_off::{count_switch, requeue, run_on};
use super::super::save_syscall_user_rsp;
use super::super::state::set_time_slice;
use super::idle::{idle_until_interrupt, select_marked_idle};

/// Voluntary-yield body. Runs with interrupts already disabled by the
/// caller. The contract backend dispatches `SwitchIntent::Yield` here.
#[inline(never)]
pub(crate) fn perform_yield_inline() {
    use crate::process::nonos_core::current_pid;

    let Some(pid) = current_pid() else { return };
    adopt_current(pid);

    let mut ctx: crate::sched::Context = unsafe { core::mem::zeroed() };
    crate::sched::Context::clear_restored_flag();
    unsafe { crate::sched::Context::save_to(&mut ctx as *mut crate::sched::Context) };
    if crate::sched::Context::was_just_restored() {
        /*
         * Resumed, possibly on another CPU: that CPU is off the stack it left.
         */
        release_leaving();
        return;
    }

    save_syscall_user_rsp(pid);
    crate::process::nonos_core::save_interrupt_context(pid, ctx);
    crate::process::nonos_core::save_fpu_state(pid);

    requeue(pid);
    set_time_slice(0);

    loop {
        /*
         * Killed from another CPU while it waited here, or its successor was
         * refused: it must not return to the call that yielded. It waits the
         * way an exiting process does, off its tables.
         */
        if is_dead(pid) {
            crate::process::exit::park_dead(pid);
        }
        if let Some(next) = select_marked_idle() {
            if next != pid {
                count_switch(next);
                switch_to_process(next);
                if is_dead(pid) {
                    continue;
                }
            } else {
                run_on(pid);
            }
            return;
        }
        idle_until_interrupt();
    }
}
