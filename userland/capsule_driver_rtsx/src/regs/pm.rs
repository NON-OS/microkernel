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

//! DMA, link and power management registers the bring-up writes
//! (rtsx_pci.h and, for the RTS522A's own copies, rtsx_pcr.h).

pub const IRQSTAT0: u16 = 0xFE21;
pub const DMATC0: u16 = 0xFE28;
pub const DMATC1: u16 = 0xFE29;
pub const DMATC2: u16 = 0xFE2A;
pub const DMATC3: u16 = 0xFE2B;
pub const DMACTL: u16 = 0xFE2C;
pub const RBCTL: u16 = 0xFE34;
pub const OBFF_CFG: u16 = 0xFE4C;
pub const PCLK_CTL: u16 = 0xFE55;
pub const PME_FORCE_CTL: u16 = 0xFE56;
pub const ASPM_FORCE_CTL: u16 = 0xFE57;
pub const PM_CLK_FORCE_CTL: u16 = 0xFE58;
pub const FUNC_FORCE_CTL: u16 = 0xFE59;
pub const CHANGE_LINK_STATE: u16 = 0xFE5B;
pub const PERST_GLITCH_WIDTH: u16 = 0xFE5C;
pub const HOST_SLEEP_STATE: u16 = 0xFE60;
pub const PM_EVENT_DEBUG: u16 = 0xFE71;
pub const NFTS_TX_CTRL: u16 = 0xFE72;
pub const PWR_GATE_CTRL: u16 = 0xFE75;
pub const LDO_PWR_SEL: u16 = 0xFE78;
pub const DUMMY_REG_RESET_0: u16 = 0xFE90;
pub const PETXCFG: u16 = 0xFF03;
pub const PM_CTRL3: u16 = 0xFF46;
pub const RTS522A_PME_FORCE_CTL: u16 = 0xFF78;
pub const RTS522A_AUTOLOAD_CFG1: u16 = 0xFF7C;
pub const RTS522A_PM_CTRL3: u16 = 0xFF7E;

pub const LINK_RDY_INT: u8 = 0x20;
pub const DMA_DONE_INT: u8 = 0x80;
pub const DMA_EN: u8 = 0x01;
pub const DMA_DIR_FROM_CARD: u8 = 0x02;
pub const DMA_512: u8 = 0x20;
pub const DMA_PACK_SIZE_MASK: u8 = 0x30;
pub const D3_DELINK_MODE_EN: u8 = 0x10;
pub const FORCE_CLKREQ_DELINK_MASK: u8 = 0x80;
pub const FORCE_CLKREQ_LOW: u8 = 0x80;
pub const LDO3318_PWR_MASK: u8 = 0x06;
pub const CD_RESUME_EN_MASK: u8 = 0xF0;
pub const FUNC_FORCE_UPME_XMT_DBG: u8 = 0x02;
pub const PME_DEBUG_0: u8 = 0x08;
pub const FORCE_ASPM_CTL0: u8 = 0x10;
pub const FORCE_ASPM_CTL1: u8 = 0x20;
