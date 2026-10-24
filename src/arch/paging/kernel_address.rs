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

//! Addresses shared kernel code hands to devices or uses to reach them.
//!
//! On aarch64 the kernel image and device registers move to the high half
//! once the kernel runs from TTBR1_EL1, so neither the address of a static
//! nor a register's physical address can be used as it is. Elsewhere both
//! are what they were, and these return their argument.

/// The bus address of a buffer that lives in the kernel image, such as a
/// static DMA ring: what a device must be given instead of the pointer.
#[inline]
pub fn kernel_dma(va: u64) -> u64 {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::mmu::image_pa(va);
    #[cfg(not(target_arch = "aarch64"))]
    va
}

/// Where the kernel reaches device registers at physical `pa` it was handed
/// raw, a BAR read from config space or a window from the device tree.
#[inline]
pub fn kernel_io(pa: u64) -> u64 {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::mmu::io(pa);
    #[cfg(not(target_arch = "aarch64"))]
    pa
}
