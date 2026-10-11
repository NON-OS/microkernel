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
//! MDIC, PHY control and SW/FW semaphore bits, Linux igb e1000_defines.h and
//! e1000_phy.h.

pub const MDIC_DATA_MASK: u32 = 0xFFFF;
pub const MDIC_REG_SHIFT: u32 = 16;
pub const MDIC_PHY_SHIFT: u32 = 21;
pub const MDIC_OP_WRITE: u32 = 1 << 26;
pub const MDIC_OP_READ: u32 = 1 << 27;
pub const MDIC_READY: u32 = 1 << 28;
pub const MDIC_ERROR: u32 = 1 << 30;
/// MAX_PHY_REG_ADDRESS: five register address bits.
pub const MAX_PHY_REG: u32 = 0x1F;
/// igb_get_phy_id_82575: "for non-SGMII PHYs (e.g. integrated copper PHYs),
/// an address of 1 should work". QEMU's igb answers address 1 only.
pub const PHY_ADDR: u32 = 1;

/// PHY_CONTROL, the IEEE BMCR, and its MII_CR_* bits.
pub const PHY_CONTROL: u32 = 0x00;
pub const MII_CR_RESTART_AUTO_NEG: u16 = 0x0200;
pub const MII_CR_POWER_DOWN: u16 = 0x0800;
pub const MII_CR_AUTO_NEG_EN: u16 = 0x1000;
/// GS40G_PAGE_SELECT, the I210/I211 PHY's page register.
pub const GS40G_PAGE_SELECT: u32 = 0x16;

pub const SWSM_SMBI: u32 = 1 << 0;
pub const SWSM_SWESMBI: u32 = 1 << 1;

/// E1000_SWFW_PHY0_SM .. PHY3_SM, by the function number in STATUS, as
/// igb_acquire_phy_82575 picks them. Firmware's copy of each sits sixteen
/// bits up (igb_acquire_swfw_sync_82575).
pub const SWFW_PHY_SM: [u32; 4] = [0x02, 0x04, 0x20, 0x40];
pub const SWFW_FW_SHIFT: u32 = 16;
