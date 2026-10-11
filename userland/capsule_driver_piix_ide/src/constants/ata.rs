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

//! ATA/ATAPI-8 ACS: the task file, the status and control bits, and the
//! commands the driver issues.

// The command block.
pub const REG_DATA: u16 = 0;
pub const REG_COUNT: u16 = 2;
pub const REG_LBA_LO: u16 = 3;
pub const REG_LBA_MID: u16 = 4;
pub const REG_LBA_HI: u16 = 5;
pub const REG_DEVICE: u16 = 6;
/// Status on read, command on write; reading it clears the pending INTRQ.
pub const REG_STATUS: u16 = 7;
pub const REG_COMMAND: u16 = 7;

pub const ST_ERR: u8 = 1 << 0;
pub const ST_DRQ: u8 = 1 << 3;
pub const ST_DF: u8 = 1 << 5;
pub const ST_BSY: u8 = 1 << 7;
/// Nothing drives the bus: no device on the channel.
pub const ST_FLOATING: u8 = 0xFF;

/// Device control (the control block, on write): nIEN keeps INTRQ down, the
/// driver polls; SRST resets both devices on the channel.
pub const CTL_NIEN: u8 = 1 << 1;
pub const CTL_SRST: u8 = 1 << 2;

/// The device register: bits 7 and 5 are obsolete ones every device
/// accepts, bit 6 LBA addressing, bit 4 the drive.
pub const DEV_BASE: u8 = 0xA0;
pub const DEV_LBA: u8 = 1 << 6;
pub const fn dev_select(drive: u8) -> u8 {
    DEV_BASE | (drive & 1) << 4
}

pub const CMD_IDENTIFY: u8 = 0xEC;
pub const CMD_READ_DMA: u8 = 0xC8;
pub const CMD_READ_DMA_EXT: u8 = 0x25;
pub const CMD_WRITE_DMA: u8 = 0xCA;
pub const CMD_WRITE_DMA_EXT: u8 = 0x35;
pub const CMD_FLUSH_CACHE: u8 = 0xE7;
pub const CMD_FLUSH_CACHE_EXT: u8 = 0xEA;

/// The signature a packet device (a CD or DVD drive) leaves in LBA mid and
/// high after a reset or a refused IDENTIFY DEVICE.
pub const SIG_PACKET: (u8, u8) = (0x14, 0xEB);

pub const SECTOR_BYTES: usize = 512;
pub const IDENTIFY_WORDS: usize = 256;
/// LBA28 addresses sectors below 2^28.
pub const LBA28_LIMIT: u64 = 1 << 28;
/// LBA48: the count register pair names up to 65536 sectors; the driver asks
/// for far fewer (`buffer::MAX_SECTORS`).
pub const LBA48_LIMIT: u64 = 1 << 48;
