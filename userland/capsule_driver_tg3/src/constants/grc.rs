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

//! The GRC block, the memory arbiter and the NVRAM arbiter (tg3.h).

pub const GRC_MODE: usize = 0x6800;
pub const GRC_MODE_WSWAP_NONFRM_DATA: u32 = 0x0000_0004;
pub const GRC_MODE_BSWAP_DATA: u32 = 0x0000_0010;
pub const GRC_MODE_WSWAP_DATA: u32 = 0x0000_0020;
pub const GRC_MODE_HOST_STACKUP: u32 = 0x0001_0000;
pub const GRC_MODE_HOST_SENDBDS: u32 = 0x0002_0000;
pub const GRC_MODE_NO_TX_PHDR_CSUM: u32 = 0x0010_0000;
pub const GRC_MODE_PCIE_PL_SEL: u32 = 0x0040_0000;
pub const GRC_MODE_NO_RX_PHDR_CSUM: u32 = 0x0080_0000;
pub const GRC_MODE_IRQ_ON_MAC_ATTN: u32 = 0x0400_0000;
pub const GRC_MODE_4X_NIC_SEND_RINGS: u32 = 0x2000_0000;
pub const GRC_MODE_PCIE_DL_SEL: u32 = 0x2000_0000;
pub const GRC_MODE_PCIE_HI_1K_EN: u32 = 0x8000_0000;
/// TL_SEL (0) | PL_SEL | DL_SEL | HI_1K_EN.
pub const GRC_MODE_PCIE_PORT_MASK: u32 =
    GRC_MODE_PCIE_PL_SEL | GRC_MODE_PCIE_DL_SEL | GRC_MODE_PCIE_HI_1K_EN;
/// tg3_init_one's tp->grc_mode on a little-endian host.
pub const GRC_MODE_BASE: u32 =
    GRC_MODE_WSWAP_DATA | GRC_MODE_BSWAP_DATA | GRC_MODE_WSWAP_NONFRM_DATA;

pub const GRC_MISC_CFG: usize = 0x6804;
pub const GRC_MISC_CFG_CORECLK_RESET: u32 = 0x0000_0001;
pub const GRC_MISC_CFG_PRESCALAR_SHIFT: u32 = 1;
/// The PCI Express bit tg3_chip_reset writes as `1 << 29` before the reset.
pub const GRC_MISC_CFG_PCIE_BIT29: u32 = 1 << 29;
pub const GRC_LOCAL_CTRL: usize = 0x6808;
pub const GRC_LCLCTRL_INT_ON_ATTN: u32 = 0x0000_0008;
pub const GRC_LCLCTRL_GPIO_UART_SEL: u32 = 0x0000_0010;
pub const GRC_LCLCTRL_GPIO_OE0: u32 = 0x0000_0800;
pub const GRC_LCLCTRL_GPIO_OE1: u32 = 0x0000_1000;
pub const GRC_LCLCTRL_GPIO_OE2: u32 = 0x0000_2000;
pub const GRC_LCLCTRL_GPIO_OUTPUT0: u32 = 0x0000_4000;
pub const GRC_LCLCTRL_GPIO_OUTPUT1: u32 = 0x0000_8000;
pub const GRC_LCLCTRL_GPIO_OUTPUT2: u32 = 0x0001_0000;
pub const GRC_LCLCTRL_AUTO_SEEPROM: u32 = 0x0100_0000;
pub const GRC_RX_CPU_EVENT: usize = 0x6810;
pub const GRC_RX_CPU_DRIVER_EVENT: u32 = 0x0000_4000;
pub const GRC_FASTBOOT_PC: usize = 0x6894;

pub const MEMARB_MODE: usize = 0x4000;
pub const MEMARB_MODE_ENABLE: u32 = 0x0000_0002;
pub const NVRAM_SWARB: usize = 0x7020;
pub const SWARB_REQ_SET1: u32 = 0x0000_0002;
pub const SWARB_REQ_CLR1: u32 = 0x0000_0020;
pub const SWARB_GNT1: u32 = 0x0000_0200;
pub const FTQ_RESET: usize = 0x5C00;
