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

//! Ordering between the CPU and a device that reads and writes memory.
//!
//! A device is not promised to be an observer in the inner shareable domain,
//! so the barriers Rust emits for `fence` on aarch64 (DMB ISH) do not order
//! what a device sees; the outer shareable forms do (Arm ARM, DDI 0487, the
//! shareability domains and DMB). A `compiler_fence` emits no instruction at
//! all. x86_64 orders write-back stores and uncached MMIO accesses as program
//! order already (Intel SDM Vol. 3A, the memory-ordering rules), so there
//! these are compiler barriers, which is what the drivers used before.

use core::sync::atomic::{compiler_fence, Ordering};

/// Every store before it is seen by the device before any store after it: a
/// descriptor before the index that hands it over, a ring before the
/// doorbell.
#[inline(always)]
pub fn dma_wmb() {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: a barrier only.
    unsafe {
        core::arch::asm!("dmb oshst", options(nostack, preserves_flags))
    };
    compiler_fence(Ordering::Release);
}

/// Every load before it completes before any access after it: the status
/// that says a buffer is done before the buffer is read.
#[inline(always)]
pub fn dma_rmb() {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: a barrier only.
    unsafe {
        core::arch::asm!("dmb oshld", options(nostack, preserves_flags))
    };
    compiler_fence(Ordering::Acquire);
}

/// Every access before it before any after it: a buffer read out before
/// the store that lets the device write it again.
#[inline(always)]
pub fn dma_mb() {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: a barrier only.
    unsafe {
        core::arch::asm!("dmb osh", options(nostack, preserves_flags))
    };
    #[cfg(not(target_arch = "aarch64"))]
    core::sync::atomic::fence(Ordering::SeqCst);
    compiler_fence(Ordering::SeqCst);
}
