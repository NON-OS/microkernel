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

use nonos_libc::{mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_MEMORY_SPACE};

use crate::error::{BgaError, BgaResult};

/// Memory decode only: the adapter never masters the bus, so it is given no
/// bus mastering to misuse.
pub fn enable_memory(device_id: u64, claim_epoch: u64) -> BgaResult<()> {
    let r =
        mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, MK_PCI_CMD_MEMORY_SPACE);
    if r < 0 {
        return Err(BgaError::BrokerCallFailed(r));
    }
    Ok(())
}
