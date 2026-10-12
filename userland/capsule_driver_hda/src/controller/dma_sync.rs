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

const LINE: u64 = 64;

/// Write `len` bytes at `va` back to memory and drop them from the cache.
#[cfg(target_arch = "x86_64")]
pub fn flush(va: u64, len: u64) {
    if len == 0 {
        return;
    }
    let mut p = va & !(LINE - 1);
    let end = va + len;
    while p < end {
        // SAFETY: CLFLUSH takes any mapped address and changes no memory.
        unsafe { core::arch::x86_64::_mm_clflush(p as *const u8) };
        p += LINE;
    }
    // SAFETY: MFENCE orders memory accesses and has no other effect.
    unsafe { core::arch::x86_64::_mm_mfence() };
}

/// Elsewhere libc's clean and invalidate does the same, stepping by the line
/// the CPU states; before, this was a no-op off x86, and a controller that
/// does not snoop played whatever stale lines memory held.
#[cfg(not(target_arch = "x86_64"))]
pub fn flush(va: u64, len: u64) {
    // SAFETY: callers pass a range inside one of this driver's DMA grants.
    unsafe { nonos_libc::mk_dma_sync_for_cpu(va as *const u8, len) }
}
