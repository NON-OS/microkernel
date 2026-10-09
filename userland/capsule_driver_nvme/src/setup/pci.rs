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
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

use crate::error::{NvmeError, NvmeResult};

/// The PCI Command bits the driver sets, all three on the broker's allowlist
/// (src/hardware/broker/pci/command.rs), which ORs them into what is there.
/// Memory Space: firmware that did not boot from this SSD can leave BAR0
/// undecoded, and then every register reads all ones (CSTS too, which the
/// ready wait takes for a device gone) and every write drops. Bus Master:
/// the queues and data move by DMA. Interrupt Disable: the driver polls,
/// so the legacy pin, shared with other functions on real chipsets, is
/// quieted; MSI-X is unaffected.
const COMMAND_BITS: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;

pub fn enable_device(device_id: u64, claim_epoch: u64) -> NvmeResult<()> {
    let r = mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, COMMAND_BITS);
    if r < 0 {
        return Err(NvmeError::BrokerCallFailed);
    }
    Ok(())
}
