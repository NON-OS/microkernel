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

use crate::regs::Regs;

/// Minimal 32-bit MMIO interface. The firmware-load path is written against
/// this instead of `Regs` directly so a host proof can drive it with a modeled
/// device (recording every write and simulating the FH completion) and check
/// the exact register sequence, with no hardware.
pub trait Mmio {
    fn read32(&self, off: usize) -> u32;
    fn write32(&self, off: usize, val: u32);
}

impl Mmio for Regs {
    fn read32(&self, off: usize) -> u32 {
        Regs::read32(self, off)
    }
    fn write32(&self, off: usize, val: u32) {
        Regs::write32(self, off, val)
    }
}
