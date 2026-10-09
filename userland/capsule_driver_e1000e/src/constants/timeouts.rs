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

//! Every wait in the bring-up, in milliseconds of uptime. Each is the Linux
//! e1000e budget it comes from, loop count times sleep, rounded up to whole
//! milliseconds; the source is named beside it.

/// reset_hw_ich8lan / reset_hw_82571: usleep_range(10000, 11000) after the
/// receive and transmit units are stopped.
pub const DMA_DRAIN_MS: u64 = 10;
/// e1000e_disable_pcie_master: MASTER_DISABLE_TIMEOUT 800 x 100 us.
pub const MASTER_DISABLE_MS: u64 = 80;
/// e1000_acquire_swflag_ich8lan: PHY_CFG_TIMEOUT for the flag to be free,
/// SW_FLAG_TIMEOUT for it to read back set.
pub const SWFLAG_FREE_MS: u64 = 100;
pub const SWFLAG_GRANT_MS: u64 = 1000;
/// e1000_get_hw_semaphore_82573: MDIO_OWNERSHIP_TIMEOUT 10 x 2 to 4 ms.
pub const MDIO_OWNERSHIP_MS: u64 = 40;
/// e1000e_read_phy_reg_mdic: E1000_GEN_POLL_TIMEOUT * 3 x 50 us.
pub const MDIC_MS: u64 = 100;
/// The pause between MDIC retries on pch_mtp and later (mdelay(10)).
pub const MDIC_RETRY_MS: u64 = 10;
/// reset_hw_ich8lan: msleep(20) after CTRL.RST, no access in between.
pub const PCH_RESET_SETTLE_MS: u64 = 20;
/// The part is not addressable for about a microsecond after CTRL.RST.
pub const RESET_SETTLE_MS: u64 = 1;
/// Bound on CTRL.RST self-clearing. Linux does not poll it; it takes
/// microseconds on a working part, so this only names a dead one.
pub const RESET_CLEAR_MS: u64 = 50;
/// e1000e_get_cfg_done_generic: mdelay(10).
pub const CFG_DONE_MS: u64 = 10;
/// e1000_lan_init_done_ich8lan: E1000_ICH8_LAN_INIT_TIMEOUT 1500 x 100 to
/// 200 us.
pub const LAN_INIT_DONE_MS: u64 = 300;
/// e1000_post_phy_reset_ich8lan: usleep_range(10000, 11000).
pub const POST_PHY_RESET_MS: u64 = 10;
/// e1000e_get_auto_rd_done: AUTO_READ_DONE_TIMEOUT 10 x 1 to 2 ms.
pub const AUTO_RD_MS: u64 = 20;
/// reset_hw_82571: msleep(25) for the PHY to load its NVM configuration.
pub const PHY_CFG_82574_MS: u64 = 25;
/// e1000_check_reset_block_ich8lan: 30 x 10 ms.
pub const RESET_BLOCK_MS: u64 = 300;
/// e1000_disable_ulp_lpt_lp: ME clears ULP_CFG_DONE within 250 x 10 ms.
pub const ULP_ME_MS: u64 = 2500;
/// msleep(50) after forcing SMBus, and after the PHY reset closing ULP exit.
pub const SMBUS_SETTLE_MS: u64 = 50;
/// e1000_toggle_lanphypc_pch_lpt: up to 21 x 5 to 6 ms for LPCD, then 30.
pub const LPCD_MS: u64 = 130;
pub const LANPHYPC_SETTLE_MS: u64 = 30;
/// The shortest sleep this clock has, standing in for Linux's 10 to 250 us
/// delays (LANPHYPC pulse, PHY reset pulse, ring flush, K1 settle).
pub const SHORT_MS: u64 = 1;
