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

pub(crate) struct Pl011 {
    base: u64,
}

impl Pl011 {
    pub(crate) fn new(base: u64) -> Self {
        Self { base }
    }

    pub(super) fn read_reg(&self, offset: u64) -> u32 {
        // SAFETY: `base` is the console window the boot path mapped as device
        // memory, and every offset used here lies inside the PL011's 4 KiB.
        unsafe { read_volatile(device::<u32>(self.base, offset)) }
    }

    pub(super) fn write_reg(&self, offset: u64, value: u32) {
        // SAFETY: as for `read_reg`.
        unsafe { write_volatile(device::<u32>(self.base, offset), value) }
    }
}
