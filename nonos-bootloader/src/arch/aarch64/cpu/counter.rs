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

//! The generic timer's virtual count, CNTVCT_EL0. It ticks at CNTFRQ_EL0, not
//! at the CPU's clock, so a count is a time, never a cycle of work. Callers
//! that time something subtract two counts; the rest only want a value that
//! moves, as the TSC is used on x86_64.

/// The virtual count now. The ISB keeps it from being read early, ahead of
/// instructions before it (Arm ARM, the generic timer's reads of CNTVCT_EL0).
#[inline(always)]
pub fn cycles() -> u64 {
    let v: u64;
    // SAFETY: CNTVCT_EL0 is readable at the EL UEFI runs at; reads no memory.
    unsafe { core::arch::asm!("isb", "mrs {}, cntvct_el0", out(reg) v, options(nomem, nostack)) };
    v
}

/// The count with an ISB on both sides, so neither earlier nor later
/// instructions are measured with it.
#[inline(always)]
pub fn cycles_serialized() -> u64 {
    let v: u64;
    // SAFETY: as above.
    unsafe {
        core::arch::asm!("isb", "mrs {}, cntvct_el0", "isb", out(reg) v, options(nomem, nostack))
    };
    v
}
