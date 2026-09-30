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

pub fn cpuid(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    let (eax, rbx, ecx, edx): (u32, u64, u32, u32);
    // SAFETY: CPUID touches no memory. RBX is restored by the exchange, which
    // stays correct when the compiler picks RBX as the output; a push/pop
    // pair would then overwrite the result with the caller's RBX.
    unsafe {
        core::arch::asm!(
            "mov {0}, rbx", "cpuid", "xchg {0}, rbx",
            out(reg) rbx, inout("eax") leaf => eax, inout("ecx") subleaf => ecx, out("edx") edx,
            options(nostack, preserves_flags)
        );
    }
    (eax, rbx as u32, ecx, edx)
}

pub fn cpuid_max_leaf() -> u32 {
    cpuid(0, 0).0
}

pub fn cpuid_max_extended_leaf() -> u32 {
    cpuid(0x80000000, 0).0
}
