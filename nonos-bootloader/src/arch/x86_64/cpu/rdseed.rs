// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! RDSEED, used only where CPUID reports it.

#[inline]
fn has_rdseed() -> bool {
    // SAFETY: CPUID leaf 7 reads no memory; rbx is saved around it.
    unsafe {
        let ebx: u32;
        core::arch::asm!(
            "push rbx",
            "mov eax, 7",
            "xor ecx, ecx",
            "cpuid",
            "mov {ebx:e}, ebx",
            "pop rbx",
            ebx = out(reg) ebx,
            out("eax") _,
            out("ecx") _,
            out("edx") _,
            options(nostack)
        );
        (ebx & (1 << 18)) != 0
    }
}

/// A seed-grade word from RDSEED, or None where it is absent or failed.
#[inline(always)]
pub fn hw_seed() -> Option<u64> {
    if !has_rdseed() {
        return None;
    }
    // SAFETY: RDSEED is present (checked above) and writes only `x`.
    unsafe {
        let mut x: u64 = 0;
        let ok = core::arch::x86_64::_rdseed64_step(&mut x);
        if ok == 1 {
            Some(x)
        } else {
            None
        }
    }
}
