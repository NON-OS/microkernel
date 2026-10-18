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
use super::broker_handles::BrokerHandles;
use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};
impl BrokerHandles {
    /// Clear Bus Master Enable, so the controller can no longer reach
    /// memory, before anything it was given is freed. Memory Space stays on
    /// until the release unmaps the window, and INTx stays off.
    ///
    /// The kernel's `MkPciConfigWrite` ORs a Command write into the
    /// register until its K6 change lands, so this write cannot clear the
    /// bit yet; the release that follows clears it with a read-back either
    /// way. Written here, as a plain store, so the order holds on its own
    /// once the kernel stores the value.
    pub fn bus_master_off(&self) -> i64 {
        let bits = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE;
        mk_pci_config_write(self.device_id, self.claim_epoch, MK_PCI_CFG_COMMAND, bits)
    }
}
