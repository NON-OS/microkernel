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

//! Streaming DMA: a write-back buffer handed to the device and taken back,
//! as Linux dma_sync_single_for_device and dma_sync_single_for_cpu. A device
//! that snoops needs neither; one that sends no-snoop requests, or sits
//! behind a remapping unit without snoop control, reads what the CPU still
//! holds in a dirty line, or the CPU reads a line it fetched before the
//! device wrote. CLFLUSH runs at any privilege level and reaches the line
//! from the grant's user mapping, so no system call is made.

use super::dma_lines::{dma_sync_lines, DMA_SYNC_LINE};

/// Before the doorbell: every line of `[ptr, ptr + len)` written back to
/// memory, so the device reads what the driver wrote.
///
/// # Safety
/// The range lies inside a live DMA grant of this process.
#[no_mangle]
pub unsafe extern "C" fn mk_dma_sync_for_device(ptr: *const u8, len: u64) {
    // SAFETY: the caller keeps the range inside a mapped grant.
    unsafe { flush(ptr as u64, len) }
}

/// After the device says it wrote: every line of `[ptr, ptr + len)` dropped
/// from the caches, so the next read comes from memory. The driver must not
/// have written the range while the device owned it.
///
/// # Safety
/// The range lies inside a live DMA grant of this process.
#[no_mangle]
pub unsafe extern "C" fn mk_dma_sync_for_cpu(ptr: *const u8, len: u64) {
    // SAFETY: as above.
    unsafe { flush(ptr as u64, len) }
}

unsafe fn flush(addr: u64, len: u64) {
    let (first, lines) = dma_sync_lines(addr, len);
    for i in 0..lines {
        let line = first + i * DMA_SYNC_LINE;
        // SAFETY: `line` holds a byte of the caller's mapped grant; CLFLUSH
        // writes the line back and drops it, and changes no memory contents.
        unsafe {
            core::arch::asm!("clflush [{}]", in(reg) line, options(nostack, preserves_flags));
        }
    }
    // SAFETY: a fence has no memory operands; it orders the flushes before
    // the doorbell store or the reads that follow (Intel SDM Vol. 2A, CLFLUSH).
    unsafe { core::arch::asm!("mfence", options(nostack, preserves_flags)) };
}
