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

//! PCI configuration words, each read and written back with only the bits
//! it is written for changed: the command word's two settings, and the one
//! helper every other configuration write of the bring-up goes through.

use nonos_libc::{
    mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_MEMORY_SPACE,
};

use crate::controller::intel::with_bits;
use crate::error::{HdaError, HdaResult};

/// Memory decoding on, for the register BAR firmware leaves off on a
/// controller it did not use, and bus mastering off. An owner that died
/// without its teardown (a panic, a kill) left its engines armed at frames
/// the kernel has freed since, so bus mastering stays off until they are
/// seen stopped and the controller reset (`enable_bus_master`). The broker
/// ORs a write naming only writable bits into the register today, so a Bus
/// Master bit firmware left set is cleared only once the kernel stores the
/// write as made; the release clears it after every owner.
pub fn enable_memory(device_id: u64, claim_epoch: u64) -> HdaResult<()> {
    let both = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;
    update(device_id, claim_epoch, MK_PCI_CFG_COMMAND, both, MK_PCI_CMD_MEMORY_SPACE)
}

/// Bus mastering on, for the rings and the stream, once every engine has
/// been seen stopped and the controller has been through a reset.
pub fn enable_bus_master(device_id: u64, claim_epoch: u64) -> HdaResult<()> {
    let bit = MK_PCI_CMD_BUS_MASTER;
    update(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bit, bit)
}

/// One PCI configuration word read, its `mask` bits set to `want`, and
/// written back when they differ. The broker refuses any other bit.
pub fn update(device_id: u64, epoch: u64, offset: u32, mask: u16, want: u16) -> HdaResult<()> {
    let cur = mk_pci_config_read(device_id, epoch, offset, 2);
    if cur < 0 {
        return Err(HdaError::BrokerCallFailed(cur));
    }
    let cur = cur as u16;
    let new = with_bits(cur, mask, want);
    if new == cur {
        return Ok(());
    }
    let r = mk_pci_config_write(device_id, epoch, offset, new);
    if r < 0 {
        return Err(HdaError::BrokerCallFailed(r));
    }
    Ok(())
}
