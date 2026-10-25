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

//! Ordering between the CPU and a device that reads and writes memory, for
//! register windows. A device is not promised to be an observer in the inner
//! shareable domain, so the barriers `fence` emits on aarch64 (DMB ISH) do
//! not order what it sees; the outer shareable forms do (Arm ARM, DDI 0487,
//! shareability domains and DMB; recalled). x86_64 keeps write-back stores
//! and uncached MMIO accesses in program order (Intel SDM Vol. 3A, memory
//! ordering), so there these are compiler barriers. The same two as
//! nonos_libc's `dma_wmb` and `dma_rmb`, here because this crate has no
//! dependencies and is proved on the host.

use core::sync::atomic::{compiler_fence, Ordering};

/// Every earlier store, DMA memory included, is seen by the device before
/// the register store that follows: a ring before its doorbell.
#[inline(always)]
pub(super) fn before_store() {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: a barrier only.
    unsafe {
        core::arch::asm!("dmb oshst", options(nostack, preserves_flags))
    };
    compiler_fence(Ordering::Release);
}

/// The register load completes before any later access: the status that
/// vouches for a buffer before the buffer is read.
#[inline(always)]
pub(super) fn after_load() {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: a barrier only.
    unsafe {
        core::arch::asm!("dmb oshld", options(nostack, preserves_flags))
    };
    compiler_fence(Ordering::Acquire);
}
