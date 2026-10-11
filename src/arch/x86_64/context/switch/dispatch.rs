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

use alloc::sync::Arc;

use crate::arch::x86_64::gdt::fs_gs;
use crate::process::core::{ProcessControlBlock, PROCESS_TABLE};
use crate::process::nonos_core::INTERRUPT_SAVED_CONTEXTS;
use crate::security::hardening::spectre_mitigations;

use super::first_entry::try_first_entry;
use super::kernel_thread::resume_kernel_thread;
use super::resume::try_resume;

pub(crate) fn switch_to_user_pcb_x86_64(pid: u32) {
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };
    // SAFETY: reading this CPU's fs base has no side effect.
    let saved_fs = unsafe { fs_gs::get_fs_base() };
    // A different process than this CPU last ran must not inherit the
    // indirect branch predictions the previous one trained.
    spectre_mitigations::switch_barrier(pcb.thread_group_id());

    // Install this task's user fs base before any of the entry paths run.
    // Threads of one process share an address space but not TLS, and the
    // iretq/sysret paths never touch MSR_FS_BASE, so the scheduler is the
    // only place the per-thread value can be carried across a switch.
    // Writing the PCB value unconditionally also scrubs whatever base the
    // previously scheduled capsule left behind.
    // SAFETY: MSR_FS_BASE only redirects user-mode fs-segment addressing;
    // the value is either zero or a user VA validated by sys_set_tls.
    unsafe { fs_gs::set_fs_base(pcb.get_tls_base()) };

    enter(&pcb, pid);
    // Back here means nothing was entered, and this CPU resumes what it ran:
    // with that process's fs base, not `pid`'s, and with the barrier owed to
    // whatever runs next.
    spectre_mitigations::switch_aborted();
    // SAFETY: the value this CPU's process had on the way in.
    unsafe { fs_gs::set_fs_base(saved_fs) };
}

/// Returns only when nothing was entered.
fn enter(pcb: &Arc<ProcessControlBlock>, pid: u32) {
    if try_first_entry(pcb, pid) {
        return;
    }
    // A pid blocked inside a syscall can have both a stale user-mode
    // trap snapshot and a live kernel resume context. The kernel
    // context must win; resuming the stale user frame re-enters at an
    // arbitrary old RIP and skips the syscall continuation entirely.
    if INTERRUPT_SAVED_CONTEXTS.read().contains_key(&pid) {
        resume_kernel_thread(pcb, pid);
        return;
    }
    if try_resume(pcb, pid) {
        return;
    }
    resume_kernel_thread(pcb, pid);
}
