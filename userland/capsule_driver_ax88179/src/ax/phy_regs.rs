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

//! The registers of the AX88179's internal PHY, at MDIO address 3
//! (AX88179_PHY_ID), by the names ax88179_178a.c and linux/mii.h give.

pub const PHY_ID: u16 = 0x03;

pub const BMCR: u16 = 0x00;
pub const BMCR_ANRESTART: u16 = 0x0200;
pub const BMCR_ANENABLE: u16 = 0x1000;

/// MII_MMD_CTRL and MII_MMD_DATA: a clause 45 register reached through
/// clause 22 (ax88179_phy_mmd_indirect).
pub const MMD_CTRL: u16 = 0x0d;
pub const MMD_DATA: u16 = 0x0e;
pub const MMD_CTRL_NOINCR: u16 = 0x4000;
/// MDIO_MMD_AN and MDIO_AN_EEE_ADV: the EEE advertisement.
pub const MMD_AN: u16 = 7;
pub const AN_EEE_ADV: u16 = 60;

/// GMII_PHY_PHYSR: the PHY's resolved speed, duplex and link.
pub const PHYSR: u16 = 0x11;
pub const PHYSR_SMASK: u16 = 0xc000;
pub const PHYSR_GIGA: u16 = 0x8000;
pub const PHYSR_100: u16 = 0x4000;
pub const PHYSR_FULL: u16 = 0x2000;
pub const PHYSR_LINK: u16 = 0x0400;

/// MII_PHYADDR, which ax88179_disable_eee writes on page 3.
pub const PHYADDR: u16 = 0x19;
pub const EEE_OFF: u16 = 0x3246;

pub const PAGE_SELECT: u16 = 0x1f;
pub const PAGE0: u16 = 0x0000;
pub const PAGE3: u16 = 0x0003;
