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

//! Block-level MMIO barriers on riscv64.

use core::sync::atomic::{compiler_fence, Ordering};

/// Every earlier write, to memory or a device, before every later one.
#[inline(always)]
pub(crate) fn fence_writes() {
    compiler_fence(Ordering::Release);
    // SAFETY: a fence orders accesses and has no other effect.
    unsafe { core::arch::asm!("fence ow, ow", options(nostack, preserves_flags)) };
}

/// Every earlier read, from memory or a device, before every later one.
#[inline(always)]
pub(crate) fn fence_reads() {
    // SAFETY: as above.
    unsafe { core::arch::asm!("fence ir, ir", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::Acquire);
}

/// Every earlier access before every later one, memory and I/O alike.
#[inline(always)]
pub(crate) fn fence_full() {
    compiler_fence(Ordering::SeqCst);
    // SAFETY: as above.
    unsafe { core::arch::asm!("fence iorw, iorw", options(nostack, preserves_flags)) };
}
