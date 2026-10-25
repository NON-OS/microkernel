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

//! Streaming DMA on riscv64. The one RISC-V machine NONOS targets so far,
//! QEMU's virt board, models no caches, so a device and the CPU see the same
//! memory and the only work is ordering: `fence iorw, iorw` (the RISC-V memory model's full
//! fence over memory and I/O) puts the driver's writes before the doorbell,
//! and the device's completion before the reads that follow.
//!
//! A part whose devices do not snoop needs cache-block operations (the Zicbom
//! extension's cbo.clean and cbo.flush) here, and a way to learn that the CPU
//! has them; neither is in this tree yet. No riscv64 driver gets a DMA grant
//! today either: without an IOMMU the kernel refuses bus mastering.

/// Before the doorbell: the driver's writes to the range ordered first.
///
/// # Safety
/// The range lies inside a live DMA grant of this process.
#[no_mangle]
pub unsafe extern "C" fn mk_dma_sync_for_device(_ptr: *const u8, _len: u64) {
    full_fence();
}

/// After the device says it wrote: the read that said so ordered before
/// every read of the range.
///
/// # Safety
/// The range lies inside a live DMA grant of this process.
#[no_mangle]
pub unsafe extern "C" fn mk_dma_sync_for_cpu(_ptr: *const u8, _len: u64) {
    full_fence();
}

fn full_fence() {
    // SAFETY: a fence orders accesses and has no other effect.
    unsafe { core::arch::asm!("fence iorw, iorw", options(nostack, preserves_flags)) };
}
