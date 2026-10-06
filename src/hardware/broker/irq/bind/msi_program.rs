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

//! Programming and clearing a function's MSI capability for one broker
//! vector, in the order Linux msi_capability_init uses: off while the message
//! is rewritten, extra vectors masked, then on with INTx disabled.

use super::msi_layout::{disable, enable_one, layout, open_first};
use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::constants::CMD_INTERRUPT_DISABLE;
use crate::drivers::pci::types::{MsiInfo, MsiMessage, PciAddress};
use crate::drivers::pci::PciError;

const PCI_COMMAND: u16 = 0x04;

pub(super) fn program(
    address: &PciAddress,
    msi: &MsiInfo,
    msg: MsiMessage,
) -> Result<(), PciError> {
    let cfg = ConfigSpace::new(*address);
    let at = layout(msi.offset as u16, msi.is_64bit, msi.per_vector_mask);
    let ctrl = cfg.read16(at.control)?;
    cfg.write16(at.control, disable(ctrl))?;
    cfg.write32(at.address_lo, msg.address as u32)?;
    if let Some(hi) = at.address_hi {
        cfg.write32(hi, (msg.address >> 32) as u32)?;
    }
    cfg.write16(at.data, msg.data as u16)?;
    if let Some(mask) = at.mask {
        cfg.write32(mask, open_first(msi.multi_message_capable))?;
    }
    // PCI Local Bus 3.0, 6.8: a function with MSI enabled must not use INTx.
    // A conventional PCI function may still assert it unless told not to.
    let cmd = cfg.read16(PCI_COMMAND)?;
    cfg.write16(PCI_COMMAND, cmd | CMD_INTERRUPT_DISABLE)?;
    cfg.write16(at.control, enable_one(ctrl))
}

/// Turns MSI off and hands INTx back, so a later INTx bind on the same
/// function finds the line it expects.
pub(super) fn clear(address: &PciAddress, msi: &MsiInfo) -> Result<(), PciError> {
    let cfg = ConfigSpace::new(*address);
    let at = layout(msi.offset as u16, msi.is_64bit, msi.per_vector_mask);
    let ctrl = cfg.read16(at.control)?;
    cfg.write16(at.control, disable(ctrl))?;
    let cmd = cfg.read16(PCI_COMMAND)?;
    cfg.write16(PCI_COMMAND, cmd & !CMD_INTERRUPT_DISABLE)
}
