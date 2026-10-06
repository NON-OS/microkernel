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

//! Handing a command buffer to the reader (rtsx_pci_send_cmd_no_wait and
//! rtsx_pci_send_cmd).

use super::wait::wait;
use crate::error::{Result, RtsxError};
use crate::regs::host::{BIPR, HCBAR, HCBCTLR};
use crate::setup::Driver;
use crate::wire::{hcbctlr, CmdBuf};

/// Copy the entries into the DMA buffer, clear what BIPR still holds from
/// the last run, and start the chip on them.
pub fn start(drv: &Driver, buf: &CmdBuf) -> Result<()> {
    if buf.overflowed() {
        return Err(RtsxError::CmdOverflow);
    }
    for (i, &word) in buf.words().iter().enumerate() {
        drv.resv.write_u32(i as u64 * 4, word);
    }
    let pending = drv.regs.r32(BIPR);
    drv.regs.w32(BIPR, pending);
    drv.regs.w32(HCBAR, drv.resv.device_addr());
    drv.regs.w32(HCBCTLR, hcbctlr(buf.words().len()));
    Ok(())
}

/// Run a command buffer to its end within `timeout_ms`.
pub fn send(drv: &Driver, buf: &CmdBuf, timeout_ms: u64) -> Result<()> {
    start(drv, buf)?;
    wait(drv, timeout_ms, RtsxError::CmdTimeout, RtsxError::CmdFailed)
}
