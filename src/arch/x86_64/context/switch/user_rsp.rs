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

//! The user stack pointer a thread parked inside a syscall had, put back in
//! this CPU's per-CPU area before its kernel context resumes, so the
//! syscall's return goes to its own user stack.

use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::process::core::ProcessControlBlock;

pub(super) fn restore_syscall_user_rsp(pcb: &Arc<ProcessControlBlock>) {
    let rsp = pcb.syscall_user_rsp.load(Ordering::Relaxed);
    if rsp == 0 {
        return;
    }
    unsafe {
        core::arch::asm!(
            "mov gs:0x28, {0}",
            in(reg) rsp,
            options(nomem, nostack, preserves_flags),
        );
    }
}
