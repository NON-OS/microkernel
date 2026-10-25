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

//! Single MMIO register accesses on riscv64. The acquire read is followed by
//! `fence i, iorw`, so nothing after it runs ahead of the device's answer; the
//! release write is preceded by `fence iorw, o`, so a descriptor written to
//! memory is visible before the doorbell that tells the device about it.

use core::sync::atomic::{compiler_fence, Ordering};

/// # Safety
/// `ptr` addresses a kernel-mapped MMIO register exactly `size_of::<T>()`
/// bytes wide and naturally aligned for `T`. One volatile load is made.
#[inline(always)]
pub(crate) unsafe fn read_relaxed<T: Copy>(ptr: *const T) -> T {
    // SAFETY: the caller has proved the pointer per the contract above.
    unsafe { core::ptr::read_volatile(ptr) }
}

/// # Safety
/// As `read_relaxed`, for one volatile store.
#[inline(always)]
pub(crate) unsafe fn write_relaxed<T: Copy>(ptr: *mut T, value: T) {
    // SAFETY: the caller has proved the pointer per the contract above.
    unsafe { core::ptr::write_volatile(ptr, value) }
}

/// # Safety
/// As `read_relaxed`.
#[inline(always)]
pub(crate) unsafe fn read_acquire<T: Copy>(ptr: *const T) -> T {
    // SAFETY: one volatile load against the proved pointer.
    let value = unsafe { core::ptr::read_volatile(ptr) };
    // SAFETY: a fence orders accesses and has no other effect.
    unsafe { core::arch::asm!("fence i, iorw", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::Acquire);
    value
}

/// # Safety
/// As `write_relaxed`.
#[inline(always)]
pub(crate) unsafe fn write_release<T: Copy>(ptr: *mut T, value: T) {
    compiler_fence(Ordering::Release);
    // SAFETY: a fence orders accesses and has no other effect.
    unsafe { core::arch::asm!("fence iorw, o", options(nostack, preserves_flags)) };
    // SAFETY: one volatile store against the proved pointer.
    unsafe { core::ptr::write_volatile(ptr, value) }
}
