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

//! What a personality needs to deliver a signal: a parked guest's registers,
//! and answering it with a whole context instead of a value.

use crate::syscall::{call_raw, N_MK_FOREIGN_CONTEXT, N_MK_FOREIGN_INTERRUPT, N_MK_FOREIGN_SIGNAL};

/// r8..r15, rdi, rsi, rbp, rbx, rdx, rax, rcx, rsp, rip, rflags: the order of
/// Linux's `struct sigcontext`, so a frame can be copied without reshuffling.
pub type ForeignRegs = [u64; 18];

/// Enter a handler: the kernel keeps the guest's FPU state until it returns.
pub const SIGNAL_DELIVER: u64 = 0;
/// Return from a handler: the FPU state saved at delivery comes back.
pub const SIGNAL_RETURN: u64 = 1;

/// The registers of a guest parked in a call, as it made the call.
pub fn mk_foreign_context(pid: u32, out: &mut ForeignRegs) -> i64 {
    call_raw(N_MK_FOREIGN_CONTEXT, [pid as u64, out.as_mut_ptr() as u64, 0, 0, 0, 0])
}

/// Wake a parked guest into `regs`. The kernel forces user selectors, keeps
/// the TLS base, masks rflags to the program's own bits, and refuses a rip or
/// rsp outside user space.
pub fn mk_foreign_signal(pid: u32, regs: &ForeignRegs, kind: u64) -> i64 {
    call_raw(N_MK_FOREIGN_SIGNAL, [pid as u64, regs.as_ptr() as u64, kind, 0, 0, 0])
}

/// Stop a guest thread that is running its own code at its next timer tick,
/// and hand it over parked, numbered `FOREIGN_NR_INTERRUPTED`, so a signal can
/// be delivered to it. 1 says it is parked in a call already, whose answer can
/// carry the signal; 0 says it will be stopped.
pub fn mk_foreign_interrupt(pid: u32) -> i64 {
    call_raw(N_MK_FOREIGN_INTERRUPT, [pid as u64, 0, 0, 0, 0, 0])
}
