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

//! Over-current protection of the card slot (rtsx_pci.h REG_OCP*), which
//! the RTS522A turns on (rts522a_init_params, ocp_en).

pub const REG_OCPCTL: u16 = 0xFD6A;
pub const REG_OCPPARA1: u16 = 0xFD6B;
pub const REG_OCPGLITCH: u16 = 0xFD6C;
pub const REG_OCPPARA2: u16 = 0xFD6D;

pub const SD_OCP_INT_EN: u8 = 0x04;
pub const SD_DETECT_EN: u8 = 0x08;
pub const SD_OCP_TIME_MASK: u8 = 0x07;
pub const SD_OCP_TIME_800: u8 = 0x05;
pub const SD_OCP_THD_MASK: u8 = 0x07;
pub const RTS522A_OCP_THD_800: u8 = 0x06;
pub const SD_OCP_GLITCH_MASK: u8 = 0x0F;
pub const SD_OCP_GLITCH_10M: u8 = 0x0F;
