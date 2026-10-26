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

//! RDRAND, used only where CPUID reports it.

#[inline]
fn has_rdrand() -> bool {
    // SAFETY: CPUID leaf 1 reads no memory; rbx is saved around it.
    unsafe {
        let ecx: u32;
        core::arch::asm!(
            "push rbx",
            "mov eax, 1",
            "cpuid",
            "mov {ecx:e}, ecx",
            "pop rbx",
            ecx = out(reg) ecx,
            out("eax") _,
            out("ecx") _,
            out("edx") _,
            options(nostack)
        );
        (ecx & (1 << 30)) != 0
    }
}

/// A word from RDRAND, or None where it is absent or failed.
#[inline(always)]
pub fn hw_random() -> Option<u64> {
    if !has_rdrand() {
        return None;
    }
    // SAFETY: RDRAND is present (checked above) and writes only registers.
    unsafe {
        let mut x: u64;
        let ok: u8;
        core::arch::asm!(
            "rdrand {x}",
            "setc {ok}",
            x = out(reg) x,
            ok = out(reg_byte) ok,
            options(nostack, nomem)
        );
        if ok != 0 {
            Some(x)
        } else {
            None
        }
    }
}
