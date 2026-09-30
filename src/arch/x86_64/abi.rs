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

//! x86_64 backend for [`ArchOps`].

use core::arch::asm;

use crate::arch::abi::ArchOps;
use crate::memory::addr::{PhysAddr, VirtAddr};

/// Zero-sized backend type. Generic code links to it through the
/// `Arch` alias in `crate::arch`.
pub struct X86_64;

impl ArchOps for X86_64 {
    #[inline(always)]
    fn halt() -> ! {
        loop {
            unsafe {
                asm!("cli; hlt", options(nomem, nostack, preserves_flags));
            }
        }
    }

    #[inline(always)]
    unsafe fn enable_interrupts() {
        asm!("sti", options(nomem, nostack, preserves_flags));
    }

    #[inline(always)]
    unsafe fn disable_interrupts() {
        asm!("cli", options(nomem, nostack, preserves_flags));
    }

    #[inline(always)]
    fn interrupts_enabled() -> bool {
        let rflags: u64;
        unsafe {
            asm!("pushfq; pop {}", out(reg) rflags, options(nomem, preserves_flags));
        }
        // RFLAGS bit 9 is IF.
        rflags & (1 << 9) != 0
    }

    // A uniprocessor boot already knows the answer, and answering from
    // memory matters: CPUID is an unconditional VM exit under hardware
    // virtualization, and this call sits on the syscall dispatch, timer
    // tick and context switch paths, so the exits dominate guest time.
    #[inline(always)]
    fn current_cpu_id() -> u32 {
        if let Some(id) = crate::smp::sole_cpu_apic_id() {
            return id;
        }
        // Read the local APIC id from CPUID leaf 1, EBX[31:24].
        // This matches the value the platform IRQ controller uses to
        // route IPIs and is the canonical CPU identifier on x86_64.
        //
        // CPUID writes RBX, which the compiler reserves and will not name
        // as an operand, so the value leaves through a scratch register and
        // RBX is restored with an exchange. The scratch register may itself
        // be RBX: the exchange is then a no-op and the result stays there,
        // which is correct because the operand is an output. A push/pop pair
        // is not: when RBX is picked as the output, the pop overwrites the
        // result with the caller's RBX, and a CPU reads a stack address as
        // its APIC id.
        let ebx: u64;
        unsafe {
            asm!(
                "mov {0}, rbx",
                "cpuid",
                "xchg {0}, rbx",
                out(reg) ebx,
                inout("eax") 1u32 => _,
                inout("ecx") 0u32 => _,
                out("edx") _,
                options(nomem, nostack, preserves_flags),
            );
        }
        (ebx as u32) >> 24
    }

    #[inline(always)]
    fn read_time_counter() -> u64 {
        // RDTSC reads the time-stamp counter into EDX:EAX. Modern
        // x86_64 CPUs run an invariant TSC; calibration to wall time
        // happens elsewhere in `sys::timer::tsc`.
        let lo: u32;
        let hi: u32;
        unsafe {
            asm!("rdtsc", out("eax") lo, out("edx") hi, options(nomem, nostack, preserves_flags));
        }
        ((hi as u64) << 32) | (lo as u64)
    }

    #[inline(always)]
    unsafe fn flush_tlb_one(addr: VirtAddr) {
        asm!("invlpg [{}]", in(reg) addr.as_u64(), options(nostack, preserves_flags));
    }

    #[inline(always)]
    unsafe fn switch_address_space(root: PhysAddr) {
        asm!("mov cr3, {}", in(reg) root.as_u64(), options(nostack, preserves_flags));
    }
}
