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

//! The part's own memory, reached through the window at
//! TG3PCI_MEM_WIN_BASE_ADDR, and what the bootcode leaves there (tg3.h).

pub const NIC_SRAM_SEND_RCB: u32 = 0x0100;
pub const NIC_SRAM_RCV_RET_RCB: u32 = 0x0200;
pub const NIC_SRAM_FIRMWARE_MBOX: u32 = 0x0B50;
pub const NIC_SRAM_FIRMWARE_MBOX_MAGIC1: u32 = 0x4B65_7654;
pub const NIC_SRAM_DATA_SIG: u32 = 0x0B54;
pub const NIC_SRAM_DATA_SIG_MAGIC: u32 = 0x4B65_7654;
pub const NIC_SRAM_DATA_CFG: u32 = 0x0B58;
pub const NIC_SRAM_DATA_CFG_LED_MODE_MASK: u32 = 0x0000_000C;
pub const NIC_SRAM_DATA_CFG_LED_MODE_MAC: u32 = 0x0000_0000;
pub const NIC_SRAM_DATA_CFG_LED_MODE_PHY_2: u32 = 0x0000_0008;
pub const NIC_SRAM_DATA_CFG_PHY_TYPE_MASK: u32 = 0x0000_0030;
pub const NIC_SRAM_DATA_CFG_PHY_TYPE_FIBER: u32 = 0x0000_0020;
pub const NIC_SRAM_DATA_CFG_ASF_ENABLE: u32 = 0x0000_0080;
pub const NIC_SRAM_DATA_CFG_EEPROM_WP: u32 = 0x0000_0100;
pub const NIC_SRAM_DATA_CFG_APE_ENABLE: u32 = 0x0020_0000;
pub const NIC_SRAM_DATA_VER: u32 = 0x0B5C;
pub const NIC_SRAM_DATA_VER_SHIFT: u32 = 16;
pub const NIC_SRAM_FW_CMD_MBOX: u32 = 0x0B78;
pub const NIC_SRAM_FW_CMD_LEN_MBOX: u32 = 0x0B7C;
pub const NIC_SRAM_FW_CMD_DATA_MBOX: u32 = 0x0B80;
pub const NIC_SRAM_FW_DRV_STATE_MBOX: u32 = 0x0C04;
pub const NIC_SRAM_DATA_CFG_2: u32 = 0x0D38;
pub const NIC_SRAM_DATA_CFG_2_APD_EN: u32 = 0x0000_4000;
/// cfg2 bit 17, TG3_PHYFLG_CAPACITIVE_COUPLING in tg3_get_eeprom_hw_cfg.
pub const NIC_SRAM_DATA_CFG_2_CAPACITIVE: u32 = 1 << 17;
pub const NIC_SRAM_DATA_CFG_3: u32 = 0x0D3C;
pub const NIC_SRAM_LNK_FLAP_AVOID: u32 = 0x0040_0000;
pub const NIC_SRAM_RX_BUFFER_DESC: u32 = 0x6000;
pub const SHASTA_EXT_LED_SHARED: u32 = 0x0000_8000;
pub const SHASTA_EXT_LED_MAC: u32 = 0x0001_0000;
pub const SHASTA_EXT_LED_COMBO: u32 = 0x0001_8000;
pub const SHASTA_EXT_LED_MODE_MASK: u32 = 0x0001_8000;

pub const DRV_STATE_START: u32 = 0x0000_0001;
pub const DRV_STATE_UNLOAD: u32 = 0x0000_0002;
pub const DRV_STATE_START_DONE: u32 = 0x8000_0001;
pub const DRV_STATE_UNLOAD_DONE: u32 = 0x8000_0002;
pub const FWCMD_NICDRV_PAUSE_FW: u32 = 0x0000_0002;
pub const FWCMD_NICDRV_ALIVE3: u32 = 0x0000_000E;
/// TG3_FW_UPDATE_TIMEOUT_SEC, sent with each heartbeat.
pub const TG3_FW_UPDATE_TIMEOUT_SEC: u32 = 5;
