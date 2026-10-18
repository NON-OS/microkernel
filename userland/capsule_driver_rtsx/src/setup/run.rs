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
//! memory decode with its interrupt pin quiet (the driver polls) and bus
//! mastering off, take the two DMA buffers, then stop the engines a dead
//! owner may have left armed before bus mastering goes on (master.rs). A
//! step that fails drops the claim, or the driver once it is built, and the
//! release takes back whatever the steps before it were granted.

use nonos_libc::{
    mk_device_claim, mk_mmio_map, mk_pci_config_write, MmioMapOut, MK_PCI_CFG_COMMAND,
};

use super::claim::Claim;
use super::discover::Found;
use super::driver::Driver;
use super::master::{master, QUIET};
use super::region::Region;
use crate::error::{Result, RtsxError};
use crate::hw::Regs;
use crate::regs::host::RESV_BUF_LEN;

/// 128 blocks per request, read or written: what one scatter-gather entry
/// here carries.
pub const DATA_BYTES: u64 = 64 * 1024;

pub fn run(found: &Found) -> Result<Driver> {
    let epoch = mk_device_claim(found.device_id);
    if epoch < 0 {
        return Err(RtsxError::Claim);
    }
    let claim_epoch = epoch as u64;
    let claim = Claim { device_id: found.device_id, epoch: claim_epoch };
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let (id, size) = (found.device_id, found.bar_size);
    if mk_mmio_map(id, claim_epoch, found.bar, 0, 0, size, &mut out) < 0 || out.length < 0x1C {
        return Err(RtsxError::MapBar);
    }
    if mk_pci_config_write(id, claim_epoch, MK_PCI_CFG_COMMAND, QUIET) < 0 {
        return Err(RtsxError::PciCommand);
    }
    let resv = Region::map(id, claim_epoch, RESV_BUF_LEN)?;
    let data = Region::map(id, claim_epoch, DATA_BYTES)?;
    let regs = Regs::new(out.user_va);
    let (family, device) = (found.family, found.device);
    let (ic_version, cur_clock, card_on) = (0, 0, false);
    master(Driver { regs, family, device, ic_version, resv, data, cur_clock, card_on, claim })
}
