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

//! The kernel's addresses for its own image and for device registers, before
//! and after it moves to the high half (TTBR1_EL1).
//!
//! The image runs first where it was loaded, translated one to one through
//! TTBR0_EL1, then at its direct-map alias, `KERNEL_SPACE_START` above its
//! physical address, where a user's TTBR0 cannot reach it. A device register
//! block moves with it. Each function answers for either place, so callers do
//! not need to know which side of the move they run on.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::geometry::KERNEL_SPACE_START;

/// Added to a device's physical address to reach its registers: zero while
/// the kernel runs one to one, `KERNEL_SPACE_START` once it runs high.
static IO_OFFSET: AtomicU64 = AtomicU64::new(0);
/// Set when the boot tables could not give some part of the image or a boot
/// device its high alias: the kernel then stays where it was loaded.
static HIGH_SHORT: AtomicBool = AtomicBool::new(false);

/// The physical address of something in the kernel image, a static or a
/// function, from either of its addresses. What a page-table descriptor,
/// TTBR, firmware call or device must be given: none of them translate.
pub fn image_pa(va: u64) -> u64 {
    if va >= KERNEL_SPACE_START {
        va - KERNEL_SPACE_START
    } else {
        va
    }
}

/// Where the kernel reaches the device registers at physical `pa`.
pub fn io(pa: u64) -> u64 {
    pa.wrapping_add(IO_OFFSET.load(Ordering::Acquire))
}

/// The kernel runs high from here on: each CPU's later register accesses go
/// to the high alias, which the boot tables already map.
pub(in crate::arch::aarch64) fn devices_high() {
    IO_OFFSET.store(KERNEL_SPACE_START, Ordering::Release);
}

/// Whether the kernel has moved to the high half.
pub fn runs_high() -> bool {
    IO_OFFSET.load(Ordering::Acquire) != 0
}

/// Something the kernel needs high was left without its alias.
pub(super) fn high_short() {
    HIGH_SHORT.store(true, Ordering::Relaxed);
}

/// Whether every block of the image and every boot device has its alias.
pub(in crate::arch::aarch64) fn high_mapped() -> bool {
    !HIGH_SHORT.load(Ordering::Relaxed)
}
