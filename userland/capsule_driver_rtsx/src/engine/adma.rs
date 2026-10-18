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

//! Starting an ADMA transfer between the data buffer and the card: one
//! scatter-gather entry after the command area, then HDBAR and HDBCTLR
//! (rtsx_pci_dma_transfer).

use crate::regs::host::{HDBAR, HDBCTLR, HOST_CMDS_BUF_LEN};
use crate::setup::Driver;
use crate::wire::{hdbctlr, sg_entry};

/// `len` bytes, from the card into the data buffer when `to_host`, from
/// the data buffer to the card otherwise. The buffer holds them all.
pub fn start_dma(drv: &Driver, len: u32, to_host: bool) {
    let entry = sg_entry(drv.data.device_addr(), len, true);
    drv.resv.write_u32(HOST_CMDS_BUF_LEN, entry as u32);
    drv.resv.write_u32(HOST_CMDS_BUF_LEN + 4, (entry >> 32) as u32);
    drv.regs.w32(HDBAR, drv.resv.device_addr() + HOST_CMDS_BUF_LEN as u32);
    drv.regs.w32(HDBCTLR, hdbctlr(to_host));
}
