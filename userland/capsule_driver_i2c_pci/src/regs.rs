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

//! The one place a register window is dereferenced. Every offset is checked
//! against the length the window was mapped with, so a computed offset that
//! strays (a GPIO pad index, a PADBAR read from dead MMIO) reads as an
//! undecoded bus, all ones, and writes nowhere, instead of touching memory
//! past the mapping. The access itself is libc's, ordered against DMA memory.

#[derive(Clone, Copy)]
pub struct Regs {
    base: *mut u8,
    len: u64,
}

impl Regs {
    /// # Safety
    /// `base` is four-byte aligned and the start of `len` bytes mapped for
    /// volatile 32-bit access (the broker's MMIO mapping, at its length),
    /// which stay mapped for as long as this value or any copy is used.
    pub const unsafe fn new(base: u64, len: u64) -> Self {
        Self { base: base as *mut u8, len }
    }

    /// `offset` as an index when a dword there lies inside the window.
    fn at(&self, offset: u64) -> Option<usize> {
        let end = offset.checked_add(4)?;
        (offset.is_multiple_of(4) && end <= self.len).then_some(offset as usize)
    }

    pub fn read32(&self, offset: u64) -> u32 {
        let Some(at) = self.at(offset) else { return u32::MAX };
        /*
         * SAFETY: `at` is dword aligned and its four bytes lie inside the
         * window, which `new`'s caller promised is mapped and aligned.
         */
        unsafe { nonos_libc::mmio_read32(self.base.add(at) as usize) }
    }

    pub fn write32(&self, offset: u64, value: u32) {
        let Some(at) = self.at(offset) else { return };
        /*
         * SAFETY: as in read32; the window is the device's own registers, so
         * the store changes nothing the driver does not own.
         */
        unsafe { nonos_libc::mmio_write32(self.base.add(at) as usize, value) }
    }
}
