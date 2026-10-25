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

//! Keeping the processor's caches and the controller's DMA in agreement.
//!
//! Intel's HD Audio controllers snoop the processor caches only while the
//! DEVC register's no-snoop bit is clear (PCI config 0x78 bit 11). Linux
//! clears it in `azx_init_pci`; this driver cannot, because the broker lets a
//! capsule write only the PCI command register. So every buffer the
//! controller reads is written back to memory after the driver fills it, and
//! every line the controller writes is dropped from the cache before the
//! driver reads it. With snooping on these flushes cost a few hundred
//! nanoseconds a period; with it off they are the difference between sound
//! and noise.

/// Write `len` bytes at `va` back to memory and drop them from the cache.
/// Every caller names part of one of this capsule's DMA grants.
pub fn flush(va: u64, len: u64) {
    if len == 0 {
        return;
    }
    flush_range(va, len);
}

#[cfg(target_arch = "x86_64")]
fn flush_range(va: u64, len: u64) {
    // CLFLUSH covers at least 64 bytes on every x86 CPU (CPUID.01H:EBX[15:8]).
    const LINE: u64 = 64;
    let mut p = va & !(LINE - 1);
    let end = va + len;
    while p < end {
        // SAFETY: CLFLUSH changes no memory, and `p` is mapped: its line
        // holds a byte of the DMA grant the caller named (`flush`).
        unsafe { core::arch::x86_64::_mm_clflush(p as *const u8) };
        p += LINE;
    }
    // SAFETY: MFENCE orders memory accesses and has no other effect.
    unsafe { core::arch::x86_64::_mm_mfence() };
}

/// Elsewhere libc's clean and invalidate does the same, stepping by the line
/// the CPU states; a no-op here would let a controller that does not snoop
/// play whatever stale lines memory held.
#[cfg(not(target_arch = "x86_64"))]
fn flush_range(va: u64, len: u64) {
    // SAFETY: callers pass a range inside one of this driver's DMA grants.
    unsafe { nonos_libc::mk_dma_sync_for_cpu(va as *const u8, len) }
}
