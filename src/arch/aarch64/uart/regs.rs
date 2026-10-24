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

use crate::arch::aarch64::mmu::pointer::device;
use core::ptr::{read_volatile, write_volatile};

/// A 16550-style register file: register `n` sits at `base + (n << shift)`
/// and is accessed `width` bytes at a time. The finder only hands over
/// widths of 1 and 4.
#[derive(Debug, Clone, Copy)]
pub(super) struct Regs {
    pub(crate) base: u64,
    pub(crate) shift: u8,
    pub(crate) width: u8,
}

impl Regs {
    fn at<T>(&self, reg: u64) -> *mut T {
        device(self.base, reg << self.shift)
    }

    pub(crate) fn read(&self, reg: u64) -> u32 {
        // SAFETY: `base` is the console window the boot path mapped as device
        // memory, and every register index used here lies inside it.
        unsafe {
            match self.width {
                4 => read_volatile(self.at::<u32>(reg)),
                _ => read_volatile(self.at::<u8>(reg)) as u32,
            }
        }
    }

    pub(crate) fn write(&self, reg: u64, value: u32) {
        // SAFETY: as for `read`.
        unsafe {
            match self.width {
                4 => write_volatile(self.at::<u32>(reg), value),
                _ => write_volatile(self.at::<u8>(reg), value as u8),
            }
        }
    }
}
