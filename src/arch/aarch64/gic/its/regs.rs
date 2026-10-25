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

//! The ITS's control frame (IHI 0069, 12.19), identity mapped at boot.

use core::ptr::{read_volatile, write_volatile};

use super::super::wait::until;
use crate::arch::aarch64::mmu::pointer::device;

pub(super) const CTLR: u64 = 0x0000;
pub(super) const TYPER: u64 = 0x0008;
pub(super) const CBASER: u64 = 0x0080;
pub(super) const CWRITER: u64 = 0x0088;
pub(super) const CREADR: u64 = 0x0090;
pub(super) const BASER: u64 = 0x0100;
/// In the translation frame, 64 KiB on: where a device writes its event.
pub(super) const TRANSLATER: u64 = 0x1_0040;

pub(super) const CTLR_ENABLED: u32 = 1 << 0;
pub(super) const CTLR_QUIESCENT: u32 = 1 << 31;

#[derive(Clone, Copy)]
pub(super) struct Regs(pub(super) u64);

impl Regs {
    pub(super) fn read32(self, offset: u64) -> u32 {
        // SAFETY: the control frame the tree described, mapped as device
        // memory at boot; `offset` is one of its registers.
        unsafe { read_volatile(device::<u32>(self.0, offset)) }
    }

    pub(super) fn write32(self, offset: u64, value: u32) {
        // SAFETY: as `read32`.
        unsafe { write_volatile(device::<u32>(self.0, offset), value) }
    }

    pub(super) fn read64(self, offset: u64) -> u64 {
        // SAFETY: as `read32`, at one of its 64-bit registers.
        unsafe { read_volatile(device::<u64>(self.0, offset)) }
    }

    pub(super) fn write64(self, offset: u64, value: u64) {
        // SAFETY: as `read64`.
        unsafe { write_volatile(device::<u64>(self.0, offset), value) }
    }

    /// Firmware may leave the ITS on; its tables and queue can only be
    /// replaced once it is off and has said it is quiescent.
    pub(super) fn quiesce(self) -> Result<(), &'static str> {
        let ctlr = self.read32(CTLR);
        if ctlr & CTLR_ENABLED != 0 {
            self.write32(CTLR, ctlr & !CTLR_ENABLED);
        }
        if until(|| self.read32(CTLR) & CTLR_QUIESCENT != 0) {
            Ok(())
        } else {
            Err("the ITS never went quiescent")
        }
    }
}
