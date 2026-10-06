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

//! Writing a grant's lines out of the CPU caches through the kernel's direct
//! map. CLFLUSH writes a dirty line back and drops it from every cache level
//! of every CPU (Intel SDM Vol. 2A, CLFLUSH), and the caches are tagged by
//! physical address, so the direct map alias reaches the lines a user
//! mapping of the same frames would.

use crate::memory::layout::DIRECTMAP_BASE;

/// Every x86-64 CPU flushes at least 64 bytes per CLFLUSH (CPUID.01H:EBX
/// bits 15:8 give 8 or more); stepping by 64 never skips a line.
const LINE: u64 = 64;

/// Flush `[phys, phys + len)` from the caches and fence, so the device and
/// any non write-back mapping see what the CPU last wrote.
pub(super) fn flush_range(phys: u64, len: u64) {
    let start = DIRECTMAP_BASE + (phys & !(LINE - 1));
    let end = DIRECTMAP_BASE + phys + len;
    let mut line = start;
    while line < end {
        // SAFETY: `line` lies in the direct map view of frames the broker
        // owns for a live grant or one being built; CLFLUSH changes no
        // memory contents.
        unsafe {
            core::arch::asm!("clflush [{}]", in(reg) line, options(nostack, preserves_flags));
        }
        line += LINE;
    }
    // SAFETY: a fence has no memory operands; it orders the flushes before
    // the mapping or doorbell that follows.
    unsafe {
        core::arch::asm!("mfence", options(nostack, preserves_flags));
    }
}

/// Say what a coherent or write-combining grant was mapped as, so the log
/// shows a ring got the memory type and the address its driver asked for.
pub(super) fn say_mapped(phys: u64, pages: u64, asked_wc: bool) {
    let wc = asked_wc && crate::arch::x86_64::pat::wc_ready();
    crate::sys::serial::print(if wc {
        b"[DMA] grant write-combining phys="
    } else {
        b"[DMA] grant uncached phys="
    });
    crate::sys::serial::print_hex(phys);
    crate::sys::serial::print(b" pages=");
    crate::sys::serial::print_dec(pages);
    crate::sys::serial::println(b"");
}
