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

//! A task switched away inside the kernel (a yield, a block, a preempt at a
//! tick) resumes where it saved its kernel context. Any user snapshot it
//! also holds is older than that point: resuming it would re-run user code
//! from an earlier instant and drop the kernel's own continuation, the
//! syscall's result with it.

use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::arch::aarch64::fpu;
use crate::process::core::{ProcessControlBlock, ProcessState, CURRENT_PID};
use crate::process::nonos_core::INTERRUPT_SAVED_CONTEXTS;
use crate::process::scheduler::preemption::{set_time_slice, DEFAULT_TIME_SLICE};

use super::address_space::swap_address_space;

/// Resume `pid` on its saved kernel context; false when it has none.
pub(super) fn try_resume_kernel(pcb: &Arc<ProcessControlBlock>, pid: u32) -> bool {
    let Some(ctx) = INTERRUPT_SAVED_CONTEXTS.write().remove(&pid) else {
        return false;
    };
    if pcb.kernel_stack_top.load(Ordering::Acquire) == 0 {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }
    if swap_address_space(pcb).is_err() {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }

    *pcb.state.lock() = ProcessState::Running;
    CURRENT_PID.store(pid, Ordering::SeqCst);
    set_time_slice(DEFAULT_TIME_SLICE);

    fpu::prepare_incoming();
    // Its own thread register, not the last task's (context::tls).
    crate::arch::aarch64::context::write_user_tls(pcb.get_tls_base());
    // The key its eventual return to EL0 installs, on this CPU, which may not
    // be the one it left (security::pac). The kernel's own key is live here.
    crate::arch::aarch64::security::enter_process(
        pcb.tgid.load(Ordering::Relaxed),
        pcb.cr3.load(Ordering::Relaxed),
    );
    ctx.restore()
}
