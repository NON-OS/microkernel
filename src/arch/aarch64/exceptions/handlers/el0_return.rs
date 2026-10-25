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

//! The switch an interrupt from EL0 may take before it returns there.

use crate::process::scheduler::contract::{switch, SwitchIntent};
use crate::process::scheduler::preemption::{clear_reschedule, need_reschedule};

/// Called last in an IRQ taken from EL0, after the interrupt is finished.
/// The interrupted context is user code, so this CPU holds no kernel lock,
/// the one place a switch cannot hand the CPU to a task that spins on a lock
/// held here. A reschedule raised by the tick (a slice spent, a woken band,
/// a thread killed from another CPU) or by a reschedule SGI is acted on now.
pub(super) fn at_el0_return() {
    if crate::smp::preempt_enabled() && need_reschedule() {
        clear_reschedule();
        // Refused only with interrupts open or no current task: then there
        // is nothing to switch from, and the request has been spent.
        let _ = switch(SwitchIntent::Preempt);
    }
    /*
     * Back here means this frame, not the snapshot taken at entry, is what
     * returns to EL0: either no switch happened, or the task came back on
     * its kernel context. A snapshot left behind would later resume it at
     * the old instant (context::switch::try_resume).
     */
    if let Some(pcb) = crate::process::current_process() {
        *pcb.saved_user_context.lock() = None;
    }
}
