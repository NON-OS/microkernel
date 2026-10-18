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

//! The slot register window the broker mapped.

use super::super::super::env::Mmio;

/// The slot's registers, mapped by the broker (uncached) at `base`. Each
/// access goes through libc's, which order it against DMA memory.
#[derive(Clone, Copy)]
pub struct MmioWindow {
    base: u64,
}

impl MmioWindow {
    /// # Safety
    /// `base` must be the start of a live broker MMIO grant covering at
    /// least the 256-byte slot register set, kept mapped while this is used.
    pub const unsafe fn new(base: u64) -> Self {
        Self { base }
    }
}

impl Mmio for MmioWindow {
    fn r8(&self, off: u32) -> u8 {
        // SAFETY: `new`'s contract keeps the 256-byte register set mapped
        // at `base`, and the engine passes only the SDHCI register offsets
        // (`sdhci::regs`), the last HOST_VERSION at 0xfe, so the access
        // lies inside it, at the register's own alignment.
        unsafe { nonos_libc::mmio_read8((self.base + off as u64) as usize) }
    }
    fn r16(&self, off: u32) -> u16 {
        // SAFETY: as in `r8`.
        unsafe { nonos_libc::mmio_read16((self.base + off as u64) as usize) }
    }
    fn r32(&self, off: u32) -> u32 {
        // SAFETY: as in `r8`.
        unsafe { nonos_libc::mmio_read32((self.base + off as u64) as usize) }
    }
    fn w8(&self, off: u32, v: u8) {
        // SAFETY: as in `r8`.
        unsafe { nonos_libc::mmio_write8((self.base + off as u64) as usize, v) }
    }
    fn w16(&self, off: u32, v: u16) {
        // SAFETY: as in `r8`.
        unsafe { nonos_libc::mmio_write16((self.base + off as u64) as usize, v) }
    }
    fn w32(&self, off: u32, v: u32) {
        // SAFETY: as in `r8`.
        unsafe { nonos_libc::mmio_write32((self.base + off as u64) as usize, v) }
    }
}
