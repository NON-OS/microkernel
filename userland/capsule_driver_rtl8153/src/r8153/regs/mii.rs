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

//! The MII registers the PHY shows at OCP_BASE_MII, and their bits
//! (IEEE 802.3 clause 22; Linux include/uapi/linux/mii.h).

pub const MII_BMCR: u16 = 0x00;
pub const MII_ADVERTISE: u16 = 0x04;
pub const MII_CTRL1000: u16 = 0x09;

pub const BMCR_RESET: u16 = 0x8000;
pub const BMCR_ANENABLE: u16 = 0x1000;
pub const BMCR_PDOWN: u16 = 0x0800;
pub const BMCR_ANRESTART: u16 = 0x0200;

pub const ADVERTISE_10HALF: u16 = 0x0020;
pub const ADVERTISE_10FULL: u16 = 0x0040;
pub const ADVERTISE_100HALF: u16 = 0x0080;
pub const ADVERTISE_100FULL: u16 = 0x0100;
pub const ADVERTISE_PAUSE_CAP: u16 = 0x0400;
pub const ADVERTISE_PAUSE_ASYM: u16 = 0x0800;

pub const ADVERTISE_1000HALF: u16 = 0x0100;
pub const ADVERTISE_1000FULL: u16 = 0x0200;
