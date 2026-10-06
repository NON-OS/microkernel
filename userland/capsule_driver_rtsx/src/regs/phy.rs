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

//! The PCIe PHY's indirect registers and the values the RTS5227 family
//! writes to them (rts5227.c, rts522a_optimize_phy, rtsx_pcr.h).

pub const PHYRWCTL: u16 = 0xFE3C;
pub const PHYDATA0: u16 = 0xFE3D;
pub const PHYDATA1: u16 = 0xFE3E;
pub const PHYADDR: u16 = 0xFE3F;

/// PHYRWCTL: start a write; the bit clears when the PHY has taken it.
pub const PHY_WRITE_START: u8 = 0x81;
pub const PHY_BUSY: u8 = 0x80;

pub const PHY_RCR1: u8 = 0x02;
pub const PHY_RCR2: u8 = 0x03;
pub const PHY_FLD0: u8 = 0x1A;
pub const PHY_FLD3: u8 = 0x1D;
pub const PHY_FLD4: u8 = 0x1E;

pub const PHY_RCR1_INIT_27S: u16 = 0x0A1F;
pub const PHY_RCR2_INIT_27S: u16 = 0xC152;
pub const PHY_FLD0_INIT_27S: u16 = 0x2546;
pub const PHY_FLD3_INIT_27S: u16 = 0x0004;
pub const PHY_FLD4_INIT_27S: u16 = 0x5C7F;
