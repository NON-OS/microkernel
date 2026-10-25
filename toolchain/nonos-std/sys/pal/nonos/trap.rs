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

// NONOS pal trap: the one place std asks the kernel for a service. Each
// architecture has its own instruction and its own registers; every other
// std file calls `trap` and never names a register. The kernel reads the
// same registers on its side (x86_64's syscall entry, aarch64's svc handler,
// riscv64's ecall handler) and libc's syscall/raw does the same, so the
// three change together.

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "riscv64")))]
compile_error!("the NONOS std layer has no system call for this architecture");

/// Ask the kernel for service `num` with six arguments (pass 0 for unused
/// ones). The answer is the kernel's, negative for an errno.
///
/// # Safety
/// Each pointer argument must be valid for what service `num` reads or
/// writes through it; the kernel checks it again on its side.
#[inline]
pub unsafe fn trap(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> i64 {
    let r: i64;
    // SAFETY: syscall enters the kernel, which reads rax and rdi..r9 and
    // writes the answer to rax; the instruction itself burns rcx and r11.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
            in("rdx") a3, in("r10") a4, in("r8") a5, in("r9") a6, out("rcx") _, out("r11") _);
    }
    // SAFETY: svc #0 traps to EL1, which reads the number from x8 and the
    // arguments from x0..x5, and writes the answer to x0.
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("svc #0", inlateout("x0") a1 => r, in("x1") a2, in("x2") a3,
            in("x3") a4, in("x4") a5, in("x5") a6, in("x8") num as u64, options(nostack));
    }
    // SAFETY: ecall traps to the supervisor, which reads the number from a7
    // and the arguments from a0..a5, and writes the answer to a0.
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("ecall", inlateout("a0") a1 => r, in("a1") a2, in("a2") a3,
            in("a3") a4, in("a4") a5, in("a5") a6, in("a7") num as u64, options(nostack));
    }
    r
}

/// Leave with service `num` (an exit), which never comes back. Should a
/// kernel answer it all the same, the process aborts rather than run on.
pub fn trap_exit(num: i64, code: u64) -> ! {
    // SAFETY: an exit reads no memory through its argument.
    unsafe { trap(num, code, 0, 0, 0, 0, 0) };
    crate::intrinsics::abort()
}
