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

//! Card-side registers: power, slot selection, pins and drive strength
//! (rtsx_pci.h).

pub const CARD_PWR_CTL: u16 = 0xFD50;
pub const CARD_SHARE_MODE: u16 = 0xFD52;
pub const CARD_DRIVE_SEL: u16 = 0xFD53;
pub const CARD_STOP: u16 = 0xFD54;
pub const CARD_OE: u16 = 0xFD55;
pub const SD30_CLK_DRIVE_SEL: u16 = 0xFD5A;
pub const CARD_DATA_SOURCE: u16 = 0xFD5B;
pub const CARD_SELECT: u16 = 0xFD5C;
pub const SD30_CMD_DRIVE_SEL: u16 = 0xFD5E;
pub const SD30_DAT_DRIVE_SEL: u16 = 0xFD5F;
pub const CARD_PULL_CTL2: u16 = 0xFD61;
pub const CARD_PULL_CTL3: u16 = 0xFD62;
pub const CARD_CLK_EN: u16 = 0xFD69;
pub const CARD_CLK_SOURCE: u16 = 0xFC2E;

pub const SD_POWER_MASK: u8 = 0x03;
pub const SD_POWER_OFF: u8 = 0x03;
pub const SD_PARTIAL_POWER_ON: u8 = 0x01;
pub const SD_POWER_ON: u8 = 0x00;
pub const PMOS_STRG_MASK: u8 = 0x10;
pub const PMOS_STRG_400MA: u8 = 0x00;
pub const SD_OUTPUT_EN: u8 = 0x04;
pub const MS_OUTPUT_EN: u8 = 0x08;
pub const SD_CLK_EN: u8 = 0x04;
pub const SD_MOD_SEL: u8 = 2;
pub const CARD_SHARE_MASK: u8 = 0x0F;
pub const CARD_SHARE_48_SD: u8 = 0x04;
pub const PINGPONG_BUFFER: u8 = 0x01;
pub const RING_BUFFER: u8 = 0x00;
pub const SD_STOP: u8 = 0x04;
pub const SD_CLR_ERR: u8 = 0x40;
pub const CRC_FIX_CLK: u8 = 0x00;
pub const SD30_VAR_CLK0: u8 = 0x04;
pub const SAMPLE_VAR_CLK1: u8 = 0x20;

/// RTSX_CARD_DRIVE_DEFAULT: MS_DRIVE_8mA | GPIO_DRIVE_8mA.
pub const CARD_DRIVE_DEFAULT: u8 = 0x41;
