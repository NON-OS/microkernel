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

//! One round: fill the sixteen vector registers, make a syscall that lets
//! another process run, read them back. All in one asm block, so nothing but
//! the kernel touches them between the fill and the read.

use core::arch::asm;

pub const SCHED_YIELD: u64 = 24;

/// CPUID says AVX and OSXSAVE, and XCR0 has SSE and AVX state on.
pub fn avx_usable() -> bool {
    let ecx = core::arch::x86_64::__cpuid(1).ecx;
    if ecx & (1 << 27) == 0 || ecx & (1 << 28) == 0 {
        return false;
    }
    let (lo, _hi): (u32, u32);
    // SAFETY: OSXSAVE is set, checked above, so XGETBV is enabled.
    unsafe { asm!("xgetbv", in("ecx") 0u32, out("eax") lo, out("edx") _hi, options(nostack)) };
    lo & 0x6 == 0x6
}

/// 16 registers of 32 bytes each, back from ymm0..ymm15.
#[target_feature(enable = "avx")]
pub unsafe fn round_ymm(pattern: &[u8; 32], out: &mut [u8; 512]) {
    // SAFETY: `pattern` is 32 readable bytes, `out` 512 writable ones; r12
    // and r13 survive the syscall, which only clobbers rax, rcx and r11.
    unsafe {
        asm!(
            ".irp r,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15",
            "vmovdqu ymm\\r, [r13]",
            ".endr",
            "mov eax, {nr}",
            "syscall",
            ".irp r,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15",
            "vmovdqu [r12], ymm\\r",
            "add r12, 32",
            ".endr",
            nr = const SCHED_YIELD,
            inout("r12") out.as_mut_ptr() => _,
            in("r13") pattern.as_ptr(),
            clobber_abi("C"),
            options(nostack),
        );
    }
}
