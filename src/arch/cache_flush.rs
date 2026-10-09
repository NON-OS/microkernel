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

//! Writing a range of memory out of the CPU data caches and dropping it there,
//! then ordering that before whatever follows (a mapping or a doorbell).
//!
//! x86_64: CLFLUSH writes a dirty line back and drops it from every level of
//! every CPU (Intel SDM Vol. 2A); every CPU flushes at least 64 bytes a line
//! (CPUID.01H:EBX bits 15:8 give 8 or more), then MFENCE.
//! aarch64: DC CIVAC cleans and invalidates to the point of coherency, one
//! line of CTR_EL0.DminLine words, then DSB SY (Arm ARM D7.5.3).

use super::cache_line_ops::{fence, flush_line};

/// Flush `[va, va + len)` from the data caches and fence.
pub fn flush_and_fence(va: u64, len: u64) {
    let step = line_bytes();
    let mut line = va & !(step - 1);
    while line < va + len {
        flush_line(line);
        line += step;
    }
    fence();
}

#[cfg(target_arch = "x86_64")]
fn line_bytes() -> u64 {
    64
}

#[cfg(target_arch = "aarch64")]
fn line_bytes() -> u64 {
    let ctr: u64;
    // SAFETY: CTR_EL0 is readable at EL1 and changes nothing.
    unsafe { core::arch::asm!("mrs {}, ctr_el0", out(reg) ctr, options(nomem, nostack)) };
    4 << ((ctr >> 16) & 0xF)
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn line_bytes() -> u64 {
    64
}
