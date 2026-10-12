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

//! Device control, status, NVM, extended control, receive address and PHY
//! power bits. Source: observed in Linux igb (e1000_defines.h), unverified
//! against Intel's datasheets.

pub const CTRL_GIO_MASTER_DISABLE: u32 = 1 << 2;
pub const CTRL_SLU: u32 = 1 << 6;
pub const CTRL_FRCSPD: u32 = 1 << 11;
pub const CTRL_FRCDPX: u32 = 1 << 12;
/// The device reset: every register back to its NVM-loaded default.
pub const CTRL_RST: u32 = 1 << 26;

pub const STATUS_FD: u32 = 1 << 0;
pub const STATUS_LU: u32 = 1 << 1;
/// Which port of the card this function is; it picks the PHY lock bit.
pub const STATUS_FUNC_MASK: u32 = 0x3 << 2;
pub const STATUS_FUNC_SHIFT: u32 = 2;
pub const STATUS_SPEED_100: u32 = 1 << 6;
pub const STATUS_SPEED_1000: u32 = 1 << 7;
pub const STATUS_GIO_MASTER_ENABLE: u32 = 1 << 19;

/// An NVM is fitted; the NVM has been read after reset.
pub const EECD_PRES: u32 = 1 << 8;
pub const EECD_AUTO_RD: u32 = 1 << 9;

/// The port's link mode, set from the NVM: copper behind the internal PHY
/// (0), 1000BASE-KX (1), SGMII (2) or SerDes (3).
pub const CTRL_EXT_LINK_MODE_MASK: u32 = 0x3 << 22;
pub const CTRL_EXT_LINK_MODE_SHIFT: u32 = 22;
/// Tells manageability firmware a driver owns the port.
pub const CTRL_EXT_DRV_LOAD: u32 = 1 << 28;

/// Address Valid: the receive address entry is live.
pub const RAH_AV: u32 = 1 << 31;

/// Go Link Disconnect in the PHY power register: while set, the PHY keeps
/// the link down.
pub const PHPM_GO_LINKD: u32 = 1 << 5;
