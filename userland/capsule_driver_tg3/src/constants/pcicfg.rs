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

//! The configuration-space registers tg3 reads and writes, and the PCI
//! Express ones tg3_chip_reset touches (pci_regs.h).

pub const TG3PCI_MISC_HOST_CTRL: u32 = 0x68;
pub const MISC_HOST_CTRL_MASK_PCI_INT: u32 = 0x0000_0002;
pub const MISC_HOST_CTRL_WORD_SWAP: u32 = 0x0000_0008;
pub const MISC_HOST_CTRL_PCISTATE_RW: u32 = 0x0000_0010;
pub const MISC_HOST_CTRL_INDIR_ACCESS: u32 = 0x0000_0080;
pub const MISC_HOST_CTRL_CHIPREV_SHIFT: u32 = 16;
/// tg3_init_one's tp->misc_host_ctrl, below the chip revision half.
pub const MISC_HOST_CTRL: u32 = MISC_HOST_CTRL_MASK_PCI_INT
    | MISC_HOST_CTRL_WORD_SWAP
    | MISC_HOST_CTRL_INDIR_ACCESS
    | MISC_HOST_CTRL_PCISTATE_RW;
/// The control bits below the revision; the broker lets exactly these move.
pub const MISC_HOST_CTRL_CONTROL: u16 = 0x03FF;

pub const TG3PCI_DMA_RW_CTRL: usize = 0x6C;
pub const DMA_RWCTRL_DIS_CACHE_ALIGNMENT: u32 = 0x0000_0001;
pub const DMA_RWCTRL_CRDRDR_RDMA_MRRS_MSK: u32 = 0x0000_0380;

pub const TG3PCI_PCISTATE: u32 = 0x70;
pub const PCISTATE_ROM_ENABLE: u16 = 0x0020;
pub const PCISTATE_ROM_RETRY_ENABLE: u16 = 0x0040;

/// Where the 57765 class keeps its real revision (TG3PCI_GEN15_PRODID_ASICREV).
pub const TG3PCI_GEN15_PRODID_ASICREV: u32 = 0xFC;
pub const TG3PCI_MEM_WIN_BASE_ADDR: usize = 0x7C;
pub const TG3PCI_MEM_WIN_DATA: usize = 0x84;

pub const PCI_CAPABILITY_LIST: u32 = 0x34;
pub const PCI_CAP_ID_EXP: u8 = 0x10;
pub const PCI_EXP_DEVCTL: u32 = 0x08;
pub const PCI_EXP_DEVCTL_RELAX_EN: u16 = 0x0010;
pub const PCI_EXP_DEVCTL_NOSNOOP_EN: u16 = 0x0800;
pub const PCI_EXP_DEVSTA: u32 = 0x0A;
/// CED | NFED | FED | URD, the bits tg3_chip_reset writes to clear them.
pub const PCI_EXP_DEVSTA_ERRORS: u16 = 0x000F;
