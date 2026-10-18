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

//! The PCI command a bring-up asks for: decoding first; bus mastering only
//! once the device has been reset, since one left live by an earlier
//! instance still holds ring addresses in frames since freed; and decoding
//! alone again before the claim goes. That last write is defence in depth:
//! the broker's release clears Bus Master itself before it frees a grant,
//! and until it stores the bits a driver writes exactly (K6) a clear here
//! changes nothing. QEMU's legacy path turns Bus Master on by itself at
//! ACKNOWLEDGE | DRIVER; its modern path does not, nor does other
//! hardware, and without it a device does no DMA and raises no MSI-X
//! message. The INTx pin is left alone: the driver may be waiting on it.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

pub const DECODE: u16 = MK_PCI_CMD_MEMORY_SPACE;
/// What a device that has been reset may also do.
pub const MASTER: u16 = DECODE | MK_PCI_CMD_BUS_MASTER;

pub fn command(device_id: u64, claim_epoch: u64, bits: u16) -> Result<(), &'static str> {
    if mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits) < 0 {
        return Err("virtio-blk: pci command setup failed");
    }
    Ok(())
}
