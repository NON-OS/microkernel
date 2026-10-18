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

//! One internal register through HAIMR, as rtsx_pci_read_register and
//! rtsx_pci_write_register do it, waiting on the clock instead of Linux's
//! 1024 polls. HAIMR never reads all ones while the reader answers (no
//! register this driver names fills bits 29:16), so all ones is a reader
//! gone from the bus, said at once rather than waited out.

use super::mmio::Regs;
use crate::clock::{now, pause, Budget};
use crate::error::{Result, RtsxError};
use crate::regs::host::HAIMR;
use crate::wire::{haimr_done, haimr_read, haimr_write};

/// Linux's 1024 polls take about a millisecond; 10 ms leaves room for a
/// slow PCIe read without hiding a dead reader.
pub const ACCESS_MS: u64 = 10;

fn finish(regs: Regs) -> Result<u32> {
    let budget = Budget::begin(now(), ACCESS_MS);
    loop {
        let value = regs.r32(HAIMR);
        if value == u32::MAX {
            return Err(RtsxError::Gone);
        }
        if haimr_done(value) {
            return Ok(value);
        }
        let t = now();
        if budget.spent(t) {
            return Err(RtsxError::RegisterTimeout);
        }
        pause(&budget, t);
    }
}

pub fn read_register(regs: Regs, reg: u16) -> Result<u8> {
    regs.w32(HAIMR, haimr_read(reg));
    Ok(finish(regs)? as u8)
}

/// The low byte reads back the data the chip took; Linux calls a
/// difference -EIO.
pub fn write_register(regs: Regs, reg: u16, mask: u8, data: u8) -> Result<()> {
    regs.w32(HAIMR, haimr_write(reg, mask, data));
    if finish(regs)? as u8 != data {
        return Err(RtsxError::RegisterMismatch);
    }
    Ok(())
}
