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

//! The controller moves every byte through its FIFOs by programmed I/O and
//! holds no DMA grant, so nothing it brings up may master the bus: not the
//! PCI function (Bus Master Enable), and not the LPSS integrated DMA engine
//! behind it, which sits in the same window and would otherwise come out of
//! reset able to reach any physical page a missing IOMMU leaves open.
//!
//! The reset bits are the LPSS chapter's RESETS_FUNC (bit 0), RESETS_APB
//! (bit 1) and RESETS_IDMA (bit 2), as recalled, unverified. They are
//! written out here rather than taken from the driver, so a driver table
//! that had them swapped is caught.

use nonos_libc::{
    take_pci_writes, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

use crate::constants::LPSS_PRIV_RESETS;
use crate::init::bring_up;
use crate::model::live;
use crate::pci::enable;
use crate::setup::{regs, SETUP};

const FUNCTION_RESETS: u32 = 0x3;
const IDMA_RESET: u32 = 1 << 2;

#[test]
fn the_pci_function_is_given_memory_space_and_never_bus_mastering() {
    let _ = take_pci_writes();
    enable(1, 1).expect("command write");
    let writes = take_pci_writes();
    assert_eq!(writes.len(), 1, "one command write");
    let (offset, value) = writes[0];
    assert_eq!(offset, MK_PCI_CFG_COMMAND);
    assert_ne!(value & MK_PCI_CMD_MEMORY_SPACE, 0, "the BAR must decode");
    assert_eq!(value & MK_PCI_CMD_BUS_MASTER, 0, "bus mastering for a driver with no DMA");
}

#[test]
fn the_integrated_dma_is_left_in_reset_and_the_function_released() {
    let bar = live();
    bring_up(regs(&bar), SETUP).expect("bring-up");
    let resets = bar.wrote32(LPSS_PRIV_RESETS as usize);
    assert_eq!(resets & IDMA_RESET, 0, "the LPSS integrated DMA was let out of reset");
    assert_eq!(resets & FUNCTION_RESETS, FUNCTION_RESETS, "the I2C function is still held");
}
