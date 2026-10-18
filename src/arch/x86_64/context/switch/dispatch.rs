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

use crate::process::core::PROCESS_TABLE;
use crate::process::nonos_core::INTERRUPT_SAVED_CONTEXTS;

use super::first_entry::try_first_entry;
use super::kernel_thread::resume_kernel_thread;
use super::resume::try_resume;

pub(crate) fn switch_to_user_pcb_x86_64(pid: u32) {
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };

    // Install this task's user fs base before any of the entry paths run.
    // Threads of one process share an address space but not TLS, and the
    // iretq/sysret paths never touch MSR_FS_BASE, so the scheduler is the
    // only place the per-thread value can be carried across a switch.
    // Writing the PCB value unconditionally also scrubs whatever base the
    // previously scheduled capsule left behind.
    // SAFETY: MSR_FS_BASE only redirects user-mode fs-segment addressing;
    // the value is either zero or a user VA validated by sys_set_tls.
    unsafe { crate::arch::x86_64::gdt::fs_gs::set_fs_base(pcb.get_tls_base()) };

    if try_first_entry(&pcb, pid) {
        return;
    }
    // A pid blocked inside a syscall can have both a stale user-mode
    // trap snapshot and a live kernel resume context. The kernel
    // context must win; resuming the stale user frame re-enters at an
    // arbitrary old RIP and skips the syscall continuation entirely.
    if INTERRUPT_SAVED_CONTEXTS.read().contains_key(&pid) {
        resume_kernel_thread(&pcb, pid);
        return;
    }
    if try_resume(&pcb, pid) {
        return;
    }
    resume_kernel_thread(&pcb, pid);
}
