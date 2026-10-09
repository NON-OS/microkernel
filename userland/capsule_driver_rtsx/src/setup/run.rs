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

//! One attempt at taking the reader: claim it, map its registers, turn on
//! memory decode and bus mastering with its interrupt pin quiet (the driver
//! polls), take the two DMA buffers, then bring the chip up.

use nonos_libc::{
    mk_device_claim, mk_mmio_map, mk_pci_config_write, MmioMapOut, MK_PCI_CFG_COMMAND,
    MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};

use super::discover::Found;
use super::driver::Driver;
use super::handles::Handles;
use super::region::Region;
use crate::error::{Result, RtsxError};
use crate::hw::Regs;
use crate::regs::host::RESV_BUF_LEN;

/// 128 blocks per read: the most one scatter-gather entry here carries.
pub const DATA_BYTES: u64 = 64 * 1024;

const COMMAND: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;

pub fn run(found: &Found) -> Result<Driver> {
    let claim = mk_device_claim(found.device_id);
    if claim < 0 {
        return Err(RtsxError::Claim);
    }
    let claim_epoch = claim as u64;
    let mut handles = Handles { device_id: found.device_id, mmio_grant: 0 };
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let (id, size) = (found.device_id, found.bar_size);
    if mk_mmio_map(id, claim_epoch, found.bar, 0, 0, size, &mut out) < 0 || out.length < 0x1C {
        return Err(RtsxError::MapBar);
    }
    handles.mmio_grant = out.grant_id;
    if mk_pci_config_write(id, claim_epoch, MK_PCI_CFG_COMMAND, COMMAND) < 0 {
        return Err(RtsxError::PciCommand);
    }
    let resv = Region::map(id, claim_epoch, RESV_BUF_LEN)?;
    let data = Region::map(id, claim_epoch, DATA_BYTES)?;
    let regs = Regs::new(out.user_va);
    let (family, device) = (found.family, found.device);
    Ok(Driver { regs, family, device, ic_version: 0, resv, data, cur_clock: 0, _handles: handles })
}
