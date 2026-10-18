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

//! Ordering against the controller on every target the capsule builds for
//! (Linux `dma_rmb` and `writel`). On x86 loads are not reordered with
//! loads and stores reach memory before a later uncached MMIO store, so a
//! full fence is more than enough. On aarch64 a `fence` orders only the
//! inner shareable domain, not the device's, and riscv64 has a fence for
//! device I/O of its own; each gets its own instruction.

/// Between reading an event's cycle bit and reading the rest of it: the
/// controller writes the cycle bit last, and the rest must not be read
/// from before it.
#[inline(always)]
pub fn read_barrier() {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: a barrier instruction; it touches no memory or register.
    unsafe {
        core::arch::asm!("dmb oshld", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "riscv64")]
    // SAFETY: a fence instruction; it touches no memory or register.
    unsafe {
        core::arch::asm!("fence r, r", options(nostack));
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
}

/// Before a doorbell write: every TRB store reaches memory the device
/// reads ahead of the MMIO store that tells it to read.
#[inline(always)]
pub fn doorbell_barrier() {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: a barrier instruction; it touches no memory or register.
    unsafe {
        core::arch::asm!("dmb oshst", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "riscv64")]
    // SAFETY: a fence instruction; it touches no memory or register.
    unsafe {
        core::arch::asm!("fence w, o", options(nostack));
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}
