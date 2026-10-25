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

// The two places a std thread meets its architecture's registers besides a
// trap: the thread pointer, through which the TLS table is found, and the
// first instructions a spawned thread runs. Kept beside trap.rs so every
// register std names lives in this directory.

/// This thread's TLS table: the word at offset 0 of the block the thread
/// pointer names, which init_tls sets to the table's own address.
#[inline]
pub fn tls_table() -> *mut *mut u8 {
    let tp: *mut *mut u8;
    // SAFETY (each arm): the thread pointer names this thread's live TLS
    // table (init_tls's invariant), so its first word is readable.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("mov {}, qword ptr fs:[0]", out(reg) tp,
            options(nostack, preserves_flags, readonly));
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("mrs {t}, tpidr_el0", "ldr {t}, [{t}]", t = out(reg) tp,
            options(nostack, preserves_flags, readonly));
    }
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("ld {}, 0(tp)", out(reg) tp,
            options(nostack, preserves_flags, readonly));
    }
    tp
}

/// riscv64 only: point `tp` at `base`, and answer 0. There `tp` is a plain
/// register the kernel's trap frame keeps, and MSTB refuses it, so it is ours.
#[cfg(target_arch = "riscv64")]
pub fn set_thread_pointer(base: u64) -> i64 {
    // SAFETY: std reaches its thread-locals only through the table, never
    // through compiler-placed TLS, so nothing else reads tp to be disturbed.
    unsafe { core::arch::asm!("mv tp, {}", in(reg) base, options(nostack, nomem)) };
    0
}

/// Where a spawned thread starts. The kernel enters it with the stack
/// pointer at the value given to MTSP, 16-aligned; the spawner left two
/// words there: at +0 the function to run, at +8 its one argument. The
/// trampoline calls the one with the other, and stops hard should it return.
///
/// # Safety
/// Only a new thread's first instruction: the stack pointer must name those
/// two words, the function one that never returns.
#[unsafe(naked)]
pub unsafe extern "C" fn thread_entry() -> ! {
    // call pushes the return address, so the callee sees the stack as the
    // x86_64 calling convention wants it at a function's first instruction.
    #[cfg(target_arch = "x86_64")]
    core::arch::naked_asm!("mov rdi, [rsp + 8]", "call qword ptr [rsp]", "ud2");
    #[cfg(target_arch = "aarch64")]
    core::arch::naked_asm!("ldr x0, [sp, #8]", "ldr x16, [sp]", "blr x16", "brk #0x1");
    #[cfg(target_arch = "riscv64")]
    core::arch::naked_asm!("ld a0, 8(sp)", "ld t0, 0(sp)", "jalr t0", "unimp");
}
