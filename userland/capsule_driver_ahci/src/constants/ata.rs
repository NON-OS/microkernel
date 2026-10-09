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

pub const ATA_IDENTIFY: u8 = 0xec;
pub const ATA_READ_DMA_EXT: u8 = 0x25;
pub const ATA_WRITE_DMA_EXT: u8 = 0x35;
pub const ATA_FLUSH_EXT: u8 = 0xea;

pub const FIS_TYPE_REG_H2D: u8 = 0x27;
pub const FIS_H2D_COMMAND: u8 = 1 << 7;
pub const FIS_H2D_LEN_DWORDS: u8 = 5;
pub const ATA_DEV_LBA: u8 = 1 << 6;

pub const CMD_HEADER_WRITE: u8 = 1 << 6;

pub const SECTOR_SIZE: usize = 512;
/// A 48-bit LBA names sectors below this. The H2D FIS has no room for more
/// address bits, so no sector at or past it can be addressed.
pub const LBA48_LIMIT: u64 = 1 << 48;
pub const MAX_SECTORS: u32 = 64;
pub const DATA_BUF_BYTES: u64 = MAX_SECTORS as u64 * SECTOR_SIZE as u64;
/// Most bytes one PRD entry can move: its DBC field holds a 22-bit byte count
/// less one (AHCI 1.3.1, 4.2.3.3).
pub const PRD_MAX_BYTES: u32 = 4 << 20;
pub const STRUCT_REGION_BYTES: u64 = 4096;
