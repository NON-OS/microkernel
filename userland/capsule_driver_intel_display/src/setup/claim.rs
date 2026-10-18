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

//! The claim, then memory decode and bus mastering on: the broker clears
//! Bus Master at a claim (section 12), and the display engine's reads of
//! the scanout surfaces are the function's own DMA. The other command bits
//! are kept as firmware left them.

use nonos_libc::{
    mk_device_claim, mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND,
    MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

use crate::error::{IntelError, IntelResult};
use crate::handles::BrokerHandles;

pub fn claim(device_id: u64) -> IntelResult<BrokerHandles> {
    let epoch = mk_device_claim(device_id);
    if epoch < 0 {
        return Err(IntelError::Broker(epoch));
    }
    let h = BrokerHandles::new(device_id, epoch as u64);
    let cmd = mk_pci_config_read(device_id, h.epoch, MK_PCI_CFG_COMMAND, 2);
    if cmd < 0 {
        return Err(IntelError::Broker(cmd));
    }
    let bits = cmd as u16 | MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;
    let rc = mk_pci_config_write(device_id, h.epoch, MK_PCI_CFG_COMMAND, bits);
    if rc < 0 {
        return Err(IntelError::Broker(rc));
    }
    Ok(h)
}

/// GMCH Graphics Control (config 0x50): the GGTT's size (`ggtt_range`).
pub fn ggc(h: &BrokerHandles) -> IntelResult<u16> {
    let v = mk_pci_config_read(h.device_id, h.epoch, 0x50, 2);
    if v < 0 {
        return Err(IntelError::Broker(v));
    }
    Ok(v as u16)
}
