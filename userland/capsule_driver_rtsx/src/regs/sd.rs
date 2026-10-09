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

//! The SD engine's registers (rtsx_pci.h, SD_CFG1 to SD_TRANSFER) and the
//! ping-pong buffer an R2 response lands in.

pub const SD_CFG1: u16 = 0xFDA0;
pub const SD_CFG2: u16 = 0xFDA1;
pub const SD_STAT1: u16 = 0xFDA3;
pub const SD_BUS_STAT: u16 = 0xFDA5;
pub const SD_SAMPLE_POINT_CTL: u16 = 0xFDA7;
pub const SD_PUSH_POINT_CTL: u16 = 0xFDA8;
pub const SD_CMD0: u16 = 0xFDA9;
pub const SD_CMD4: u16 = 0xFDAD;
pub const SD_BYTE_CNT_L: u16 = 0xFDAF;
pub const SD_BYTE_CNT_H: u16 = 0xFDB0;
pub const SD_BLOCK_CNT_L: u16 = 0xFDB1;
pub const SD_BLOCK_CNT_H: u16 = 0xFDB2;
pub const SD_TRANSFER: u16 = 0xFDB3;
pub const PPBUF_BASE2: u16 = 0xFA00;

pub const SD_CLK_DIVIDE_MASK: u8 = 0xC0;
pub const SD_CLK_DIVIDE_0: u8 = 0x00;
pub const SD_CLK_DIVIDE_128: u8 = 0x80;
pub const SD_BUS_WIDTH_MASK: u8 = 0x03;
pub const SD_BUS_WIDTH_1BIT: u8 = 0x00;
pub const SD_BUS_WIDTH_4BIT: u8 = 0x01;
pub const SD_MODE_MASK: u8 = 0x0C;
pub const SD_20_MODE: u8 = 0x00;

pub const SD_CMD_START: u8 = 0x40;
pub const SD_TRANSFER_START: u8 = 0x80;
pub const SD_TRANSFER_END: u8 = 0x40;
pub const SD_STAT_IDLE: u8 = 0x20;
pub const SD_TM_CMD_RSP: u8 = 0x08;
pub const SD_TM_AUTO_READ_2: u8 = 0x0E;

/// SD_CFG2 response types (SD_RSP_TYPE_R0 and on).
pub const SD_RSP_TYPE_R0: u8 = 0x04;
pub const SD_RSP_TYPE_R1: u8 = 0x01;
pub const SD_RSP_TYPE_R1B: u8 = 0x09;
pub const SD_RSP_TYPE_R2: u8 = 0x02;
pub const SD_RSP_TYPE_R3: u8 = 0x05;
pub const SD_NO_CHECK_CRC7: u8 = 0x04;
pub const SD_NO_CHECK_WAIT_CRC_TO: u8 = 0x20;
/// SD_STAT1.
pub const SD_CRC7_ERR: u8 = 0x80;

/// SD_BUS_STAT.
pub const SD_CLK_TOGGLE_EN: u8 = 0x80;
pub const SD20_RX_SEL_MASK: u8 = 0x08;
pub const SD20_RX_POS_EDGE: u8 = 0x00;
