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

use crate::error::{XhciError, XhciResult};

/// At claim: Memory Space, so the registers decode, and INTx off. Firmware
/// that never used the controller can leave Memory Space clear, and then
/// every register access drops without an error. INTx stays off: the driver
/// takes MSI-X or MSI or polls, never the legacy line; with USBCMD.INTE set
/// and INTx enabled the controller would hold a level-triggered line,
/// possibly shared, that nothing acknowledges. Bus Master stays off.
pub fn enable_decode(device_id: u64, claim_epoch: u64) -> XhciResult<()> {
    command(device_id, claim_epoch, MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE)
}

/// Bus Master too, only once `take_over` saw the controller halted and
/// reset with CNR clear and `assemble` allocated its memory, and before
/// `program` hands it any. A predecessor that died without its `Drop` can
/// leave it running on rings armed at frames since freed; on before the
/// halt, it could DMA there first, and on before the allocation, a refusal
/// there released it on. Off at the ERSTBA write, the controller's fetch of
/// the event ring segment table fails: qemu-xhci shows HCE and answers no
/// command (xHCI 1.2 section 5.5.2.3.2).
pub fn enable_bus_master(device_id: u64, claim_epoch: u64) -> XhciResult<()> {
    let bits = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE;
    command(device_id, claim_epoch, bits)
}

fn command(device_id: u64, claim_epoch: u64, bits: u16) -> XhciResult<()> {
    let r = mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits);
    if r < 0 {
        return Err(XhciError::BrokerCallFailed(r));
    }
    Ok(())
}
