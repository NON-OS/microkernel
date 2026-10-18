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

/*
 * Issuing a hypercall. The hypercall MSR is enabled with a page, which the
 * hypervisor fills with the instruction for this CPU; the call is that
 * instruction issued here, VMCALL on Intel and VMMCALL on AMD and Hygon,
 * so no kernel page is ever made executable for the hypervisor's code.
 */

use core::arch::asm;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::arch::x86_64::cpu::cpuid;

static AMD: AtomicBool = AtomicBool::new(false);

/// "Auth" of AuthenticAMD and "Hygo" of HygonGenuine, CPUID 0's EBX.
const EBX_AMD: u32 = 0x6874_7541;
const EBX_HYGON: u32 = 0x6F67_7948;

pub(super) fn choose_instruction() {
    let ebx = cpuid(0).1;
    AMD.store(ebx == EBX_AMD || ebx == EBX_HYGON, Ordering::Relaxed);
}

/// RAX after the call: the status in its low sixteen bits.
pub(super) fn call(control: u64, input: u64, output: u64) -> u64 {
    let rax: u64;
    // SAFETY: a hypercall, after the hypercall MSR was enabled. The input
    // and output are guest addresses of pages this module owns, or the
    // value itself for a fast call; the hypervisor clobbers the volatile
    // registers declared here and nothing else.
    unsafe {
        if AMD.load(Ordering::Relaxed) {
            asm!(
                "vmmcall",
                inout("rcx") control => _,
                inout("rdx") input => _,
                inout("r8") output => _,
                out("rax") rax,
                out("r9") _,
                out("r10") _,
                out("r11") _,
                options(nostack),
            );
        } else {
            asm!(
                "vmcall",
                inout("rcx") control => _,
                inout("rdx") input => _,
                inout("r8") output => _,
                out("rax") rax,
                out("r9") _,
                out("r10") _,
                out("r11") _,
                options(nostack),
            );
        }
    }
    rax
}
