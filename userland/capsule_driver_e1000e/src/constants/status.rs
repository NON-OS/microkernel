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

//! STATUS, EECD, EXTCNF_CTRL, FWSM and H2ME bits, values from Linux e1000e
//! defines.h and ich8lan.h.

pub const STATUS_FD: u32 = 1 << 0;
pub const STATUS_LU: u32 = 1 << 1;
pub const STATUS_SPEED_SHIFT: u32 = 6;
pub const STATUS_SPEED_MASK: u32 = 3 << STATUS_SPEED_SHIFT;
pub const STATUS_LAN_INIT_DONE: u32 = 1 << 9;
pub const STATUS_PHYRA: u32 = 1 << 10;
pub const STATUS_GIO_MASTER_ENABLE: u32 = 1 << 19;

pub const EECD_AUTO_RD: u32 = 1 << 9;

/// E1000_EXTCNF_CTRL_SWFLAG on PCH parts and _MDIO_SW_OWNERSHIP on the
/// 82574 and 82583: the same bit, defines.h gives both names 0x20.
pub const EXTCNF_CTRL_SWFLAG: u32 = 1 << 5;
pub const EXTCNF_CTRL_GATE_PHY_CFG: u32 = 1 << 7;

pub const FWSM_RSPCIPHY: u32 = 1 << 6;
pub const FWSM_ULP_CFG_DONE: u32 = 1 << 10;
pub const FWSM_FW_VALID: u32 = 1 << 15;

pub const H2ME_ULP: u32 = 1 << 11;
pub const H2ME_ENFORCE_SETTINGS: u32 = 1 << 12;
