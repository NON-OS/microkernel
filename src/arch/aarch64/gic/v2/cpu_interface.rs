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

//! The GICv2 CPU interface: memory mapped, banked per CPU at one address.

use crate::arch::aarch64::mmu::io;
use core::ptr::{read_volatile, write_volatile};

const GICC_CTLR: u64 = 0x00;
const GICC_PMR: u64 = 0x04;
const GICC_IAR: u64 = 0x0C;
const GICC_EOIR: u64 = 0x10;

/// Bit 0 enables signalling: group 1 from the non-secure side, which is what
/// firmware leaves an OS on a Pi, and group 0 on a GIC without security, as
/// QEMU's is. EOImode stays 0, so one EOIR write drops and deactivates.
const CTLR_ENABLE: u32 = 1;
/// Let every priority through; GICs with fewer priority bits round down.
const PMR_ALL: u32 = 0xFF;

pub(super) struct CpuInterface {
    base: u64,
}

impl CpuInterface {
    pub(super) fn new(base: u64) -> Self {
        Self { base }
    }

    pub(super) fn enable(&self) {
        self.write(GICC_PMR, PMR_ALL);
        self.write(GICC_CTLR, CTLR_ENABLE);
    }

    pub(super) fn acknowledge(&self) -> u32 {
        // SAFETY: the base came from the device tree or the board default and
        // was mapped as device memory at boot; IAR is a 32-bit register.
        unsafe { read_volatile((io(self.base) + GICC_IAR) as *const u32) }
    }

    pub(super) fn end(&self, raw: u32) {
        self.write(GICC_EOIR, raw);
    }

    fn write(&self, offset: u64, value: u32) {
        // SAFETY: as in `acknowledge`; every offset here is a 32-bit register
        // inside the CPU interface's first page.
        unsafe { write_volatile((io(self.base) + offset) as *mut u32, value) }
    }
}
