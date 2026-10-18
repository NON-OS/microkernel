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
    mk_pci_config_read, mk_pci_config_update, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
};

use super::broker_handles::BrokerHandles;

impl BrokerHandles {
    /// Clear Bus Master Enable in the PCI command register, keeping its other
    /// bits, by the masked form of the write: the plain form ORs a word of
    /// writable command bits onto the register (src/hardware/broker/pci/
    /// change.rs) and could not clear it. Nothing is written when the
    /// register cannot be read; the release that follows clears it again,
    /// before any grant is torn down.
    pub(super) fn bus_master_off(&self) {
        let (dev, epoch) = (self.device_id, self.claim_epoch);
        if mk_pci_config_read(dev, epoch, MK_PCI_CFG_COMMAND, 2) >= 0 {
            let bit = MK_PCI_CMD_BUS_MASTER;
            let _ = mk_pci_config_update(dev, epoch, MK_PCI_CFG_COMMAND, bit, 0);
        }
    }
}
