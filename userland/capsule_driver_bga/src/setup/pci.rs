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

use nonos_libc::{
    mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_MEMORY_SPACE,
};

use crate::error::{BgaError, BgaResult};

/// Memory decoding on, for the register and framebuffer BARs, and bus
/// mastering off: the adapter does no DMA. The other command bits are kept
/// as read. The broker ORs a write that names only writable bits into the
/// register today, so a Bus Master bit firmware left set is cleared only
/// once the kernel stores the write as made (K6); the adapter has no DMA
/// engine to use it either way.
pub fn enable_memory(device_id: u64, claim_epoch: u64) -> BgaResult<()> {
    let cur = mk_pci_config_read(device_id, claim_epoch, MK_PCI_CFG_COMMAND, 2);
    if cur < 0 {
        return Err(BgaError::BrokerCallFailed(cur));
    }
    let cmd = (cur as u16 | MK_PCI_CMD_MEMORY_SPACE) & !MK_PCI_CMD_BUS_MASTER;
    let r = mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, cmd);
    if r < 0 {
        return Err(BgaError::BrokerCallFailed(r));
    }
    Ok(())
}
