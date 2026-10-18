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

//! Where shared kernel code reaches device registers.
//!
//! On aarch64 device registers move to the high half once the kernel runs
//! from TTBR1_EL1, so a register's physical address cannot be used as it
//! is. Elsewhere it is what it was, and this returns its argument. (The
//! kernel's one static DMA ring, virtio-rng's, went with that driver: K9.)

/// Where the kernel reaches device registers at physical `pa` it was handed
/// raw, a BAR read from config space or a window from the device tree.
#[inline]
pub fn kernel_io(pa: u64) -> u64 {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::mmu::io(pa);
    #[cfg(not(target_arch = "aarch64"))]
    pa
}
