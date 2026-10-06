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
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

use crate::error::{AhciError, AhciResult};

/// Turn on Memory Space decode and Bus Master. Firmware that booted from
/// another disk, or from none, may leave Memory Space off on the SATA
/// controller, and then every ABAR access is dropped: reads give all ones
/// and the HBA looks like 32 ports that never come up. The broker ORs these
/// bits into the Command register; it allows no others.
pub fn enable_decode_and_dma(device_id: u64, claim_epoch: u64) -> AhciResult<()> {
    let bits = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;
    let r = mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits);
    if r < 0 {
        return Err(AhciError::BrokerCallFailed(r));
    }
    Ok(())
}
