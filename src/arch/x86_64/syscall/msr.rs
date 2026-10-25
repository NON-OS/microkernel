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

pub use super::msr_regs::*;
pub use super::msr_star::setup_star;

#[inline]
pub fn read_msr(msr_addr: u32) -> u64 {
    unsafe {
        let lo: u32;
        let hi: u32;
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr_addr,
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack, preserves_flags)
        );
        ((hi as u64) << 32) | (lo as u64)
    }
}

#[inline]
pub fn write_msr(msr_addr: u32, value: u64) {
    unsafe {
        let lo = value as u32;
        let hi = (value >> 32) as u32;
        core::arch::asm!(
            "wrmsr",
            in("ecx") msr_addr,
            in("eax") lo,
            in("edx") hi,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub fn setup_lstar(entry_point: u64) {
    write_msr(IA32_LSTAR, entry_point);
}

pub fn setup_fmask() {
    write_msr(IA32_FMASK, SYSCALL_FMASK);
}

pub fn enable_sce() {
    let mut efer = read_msr(IA32_EFER);
    efer |= EFER_SCE;
    write_msr(IA32_EFER, efer);
}

pub fn is_sce_enabled() -> bool {
    let efer = read_msr(IA32_EFER);
    (efer & EFER_SCE) != 0
}
