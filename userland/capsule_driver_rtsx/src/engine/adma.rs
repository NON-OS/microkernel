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

//! Starting an ADMA read into the data buffer: one scatter-gather entry
//! after the command area, then HDBAR and HDBCTLR (rtsx_pci_dma_transfer).

use crate::regs::host::{HDBAR, HDBCTLR, HOST_CMDS_BUF_LEN};
use crate::setup::Driver;
use crate::wire::{hdbctlr_read, sg_entry};

/// `len` bytes from the card into the data buffer, which holds them all.
pub fn start_read(drv: &Driver, len: u32) {
    let entry = sg_entry(drv.data.device_addr(), len, true);
    drv.resv.write_u32(HOST_CMDS_BUF_LEN, entry as u32);
    drv.resv.write_u32(HOST_CMDS_BUF_LEN + 4, (entry >> 32) as u32);
    drv.regs.w32(HDBAR, drv.resv.device_addr() + HOST_CMDS_BUF_LEN as u32);
    drv.regs.w32(HDBCTLR, hdbctlr_read());
}
