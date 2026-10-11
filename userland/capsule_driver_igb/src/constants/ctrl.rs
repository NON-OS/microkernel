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
//! Device control, status, NVM, extended control and receive-address bits,
//! Linux igb e1000_defines.h.

pub const CTRL_GIO_MASTER_DISABLE: u32 = 1 << 2;
pub const CTRL_SLU: u32 = 1 << 6;
pub const CTRL_FRCSPD: u32 = 1 << 11;
pub const CTRL_FRCDPX: u32 = 1 << 12;
/// E1000_CTRL_RST, the global reset igb_reset_hw_82575 and _82580 issue.
pub const CTRL_RST: u32 = 1 << 26;

pub const STATUS_FD: u32 = 1 << 0;
pub const STATUS_LU: u32 = 1 << 1;
/// E1000_STATUS_FUNC_MASK and _SHIFT: which port of the card this function
/// is, which picks its PHY semaphore bit.
pub const STATUS_FUNC_MASK: u32 = 0x3 << 2;
pub const STATUS_FUNC_SHIFT: u32 = 2;
pub const STATUS_SPEED_100: u32 = 1 << 6;
pub const STATUS_SPEED_1000: u32 = 1 << 7;
pub const STATUS_GIO_MASTER_ENABLE: u32 = 1 << 19;

/// E1000_EECD_PRES: an NVM is fitted. E1000_EECD_AUTO_RD: it has been read.
pub const EECD_PRES: u32 = 1 << 8;
pub const EECD_AUTO_RD: u32 = 1 << 9;

/// E1000_CTRL_EXT_LINK_MODE_MASK: copper behind the internal PHY (GMII, 0),
/// 1000BASE-KX (1), SGMII (2) or SerDes (3).
pub const CTRL_EXT_LINK_MODE_MASK: u32 = 0x3 << 22;
pub const CTRL_EXT_DRV_LOAD: u32 = 1 << 28;

pub const RAH_AV: u32 = 1 << 31;

/// E1000_82580_PM_GO_LINKD in E1000_82580_PHY_POWER_MGMT: while set, the
/// PHY keeps the link down.
pub const PHPM_GO_LINKD: u32 = 1 << 5;
