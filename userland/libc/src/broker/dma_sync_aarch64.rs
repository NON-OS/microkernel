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

//! Streaming DMA on aarch64, for parts whose devices do not snoop the CPU's
//! caches; on a coherent part they cost little, so a driver always calls them.
//!
//! From the Arm ARM (DDI 0487): DC CVAC writes a dirty line back to the point
//! of coherency, DC CIVAC does that and drops the line. Both run at EL0 once
//! the kernel sets SCTLR_EL1.UCI, and act only on an address EL0 may read, so
//! a capsule can touch no line but its own grant's. DC IVAC, which would drop
//! a dirty line unwritten, is EL1 only; `for_cpu` cleans as well, and a
//! driver that kept the rule (no CPU write while the device owns the range)
//! has nothing dirty there to write. DSB SY completes the maintenance before
//! the doorbell write after it, or the loads that follow.

use super::dma_lines::{ctr_dminline_bytes, dma_sync_lines_of};

/// Before the doorbell: every line of `[ptr, ptr + len)` written back to
/// memory, so the device reads what the driver wrote.
///
/// # Safety
/// The range lies inside a live DMA grant of this process.
#[no_mangle]
pub unsafe extern "C" fn mk_dma_sync_for_device(ptr: *const u8, len: u64) {
    each_line(ptr as u64, len, |line| {
        // SAFETY: `line` holds a byte of the caller's mapped grant; DC CVAC
        // writes it back and changes no memory contents.
        unsafe { core::arch::asm!("dc cvac, {}", in(reg) line, options(nostack, preserves_flags)) }
    });
    dsb_sy();
}

/// After the device says it wrote: every line of `[ptr, ptr + len)` dropped
/// from the caches, so the next read comes from memory. The driver must not
/// have written the range while the device owned it.
///
/// # Safety
/// The range lies inside a live DMA grant of this process.
#[no_mangle]
pub unsafe extern "C" fn mk_dma_sync_for_cpu(ptr: *const u8, len: u64) {
    // The read that said the device was done comes first.
    dsb_sy();
    each_line(ptr as u64, len, |line| {
        // SAFETY: as above; DC CIVAC writes back and drops the line.
        unsafe { core::arch::asm!("dc civac, {}", in(reg) line, options(nostack, preserves_flags)) }
    });
    dsb_sy();
}

fn each_line(addr: u64, len: u64, mut op: impl FnMut(u64)) {
    let ctr: u64;
    // SAFETY: CTR_EL0 is readable at EL0 once the kernel sets SCTLR_EL1.UCT.
    unsafe { core::arch::asm!("mrs {}, ctr_el0", out(reg) ctr, options(nomem, nostack)) };
    let line = ctr_dminline_bytes(ctr);
    let (first, lines) = dma_sync_lines_of(addr, len, line);
    (0..lines).for_each(|i| op(first + i * line));
}

fn dsb_sy() {
    // SAFETY: a barrier only.
    unsafe { core::arch::asm!("dsb sy", options(nostack, preserves_flags)) }
}
