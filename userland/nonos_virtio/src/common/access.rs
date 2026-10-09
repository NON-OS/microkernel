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

//! The common configuration as the bring-up reaches it.

use crate::mmio::Mmio;

/// Register reads and writes at byte offsets into the common configuration.
pub trait CommonCfg {
    fn r8(&self, off: usize) -> u8;
    fn r16(&self, off: usize) -> u16;
    fn r32(&self, off: usize) -> u32;
    fn w8(&self, off: usize, value: u8);
    fn w16(&self, off: usize, value: u16);
    fn w32(&self, off: usize, value: u32);

    /// A 64-bit register as two 32-bit halves, low first. QEMU's common
    /// configuration takes at most 4 bytes per access.
    fn w64(&self, off: usize, value: u64) {
        self.w32(off, value as u32);
        self.w32(off + 4, (value >> 32) as u32);
    }
}

impl CommonCfg for Mmio {
    fn r8(&self, off: usize) -> u8 {
        Mmio::r8(*self, off)
    }
    fn r16(&self, off: usize) -> u16 {
        Mmio::r16(*self, off)
    }
    fn r32(&self, off: usize) -> u32 {
        Mmio::r32(*self, off)
    }
    fn w8(&self, off: usize, value: u8) {
        Mmio::w8(*self, off, value)
    }
    fn w16(&self, off: usize, value: u16) {
        Mmio::w16(*self, off, value)
    }
    fn w32(&self, off: usize, value: u32) {
        Mmio::w32(*self, off, value)
    }
}
