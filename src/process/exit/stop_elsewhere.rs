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

use crate::process::core::Pid;

/*
 * Killed while another CPU runs it: that CPU would go on running its user
 * code, and holding its stack and tables, until its next tick. The IPI
 * raises that CPU's reschedule flag, and ends its halt if it waits in a
 * yield; the scheduler there then switches away from a zombie rather than
 * back into it (see `preempt_current_process` and `perform_yield_inline`).
 */
pub(super) fn stop_elsewhere(pid: Pid) {
    let Some(cpu) = crate::process::scheduler::selection::cpu_running(pid) else {
        return;
    };
    if cpu != crate::smp::cpu_id() {
        crate::smp::send_reschedule_ipi(cpu);
    }
}
