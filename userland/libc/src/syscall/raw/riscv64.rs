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

//! The riscv64 side of a system call.
//!
//! `ecall` traps to the supervisor, where the kernel reads the call number
//! from `a7` and the arguments from `a0` through `a5`, and leaves the result
//! in `a0`. That is the contract `arch::riscv64::interrupts::handlers::syscall`
//! reads, so the two have to be changed together. Six arguments, as on the
//! other architectures, because the kernel's `SyscallArgs` carries six.

/// Trap into the kernel.
///
/// # Safety
/// The caller is invoking a kernel service: whether the arguments are sound is
/// the caller's business, and the kernel validates them again on its side.
#[inline]
pub(in crate::syscall) unsafe fn raw(
    num: i64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
) -> i64 {
    let ret: i64;
    // SAFETY: `ecall` is the architected way to ask the supervisor for a
    // service. The operands name the registers the kernel reads, a0 is written
    // back with the result, and the trap frame restores every other register,
    // so nothing else is clobbered. The call may touch memory the arguments
    // point at, so it is not marked `nomem`.
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") a1 => ret,
            in("a1") a2,
            in("a2") a3,
            in("a3") a4,
            in("a4") a5,
            in("a5") a6,
            in("a7") num as u64,
            options(nostack)
        );
    }
    ret
}
