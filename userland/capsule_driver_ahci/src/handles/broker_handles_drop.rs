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
    mk_device_release, mk_irq_unbind, mk_mmio_unmap, mk_pci_config_write, MK_PCI_CFG_COMMAND,
    MK_PCI_CMD_MEMORY_SPACE,
};

use super::broker_handles::BrokerHandles;

impl Drop for BrokerHandles {
    fn drop(&mut self) {
        // Bus Master off before the claim goes back. The broker ORs Command
        // bits until K6, so this write changes nothing yet, and the release's
        // own stop, read back before it frees the DMA grants, is what holds;
        // it is written so the order holds on its own once the broker stores
        // the word.
        let word = MK_PCI_CMD_MEMORY_SPACE;
        let _ = mk_pci_config_write(self.device_id, self.claim_epoch, MK_PCI_CFG_COMMAND, word);
        let _ = mk_irq_unbind(self.irq_grant_id);
        let _ = mk_mmio_unmap(self.mmio_grant_id);
        let _ = mk_device_release(self.device_id);
    }
}
