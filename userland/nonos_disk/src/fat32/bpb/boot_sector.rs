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

//! The FAT32 boot sector: BIOS parameter block plus the extended block. The
//! geometry fields are what a driver actually uses; the legacy CHS values
//! are the conventional 63 and 255 that nothing reads and everything
//! expects to be there.

use alloc::vec::Vec;

use super::super::geometry::{Geometry, FAT_COUNT, RESERVED_SECTORS};
use crate::sink::SECTOR_SIZE;

pub const BACKUP_BOOT_SECTOR: u64 = 6;
pub const VOLUME_LABEL: &[u8; 11] = b"NONOS-ESP  ";

pub fn boot_sector(geo: &Geometry, hidden_sectors: u64, volume_id: u32) -> Vec<u8> {
    let mut s = alloc::vec![0u8; SECTOR_SIZE];
    s[0..3].copy_from_slice(&[0xEB, 0x58, 0x90]);
    s[3..11].copy_from_slice(b"NONOS   ");
    s[11..13].copy_from_slice(&(SECTOR_SIZE as u16).to_le_bytes());
    s[13] = geo.sectors_per_cluster;
    s[14..16].copy_from_slice(&(RESERVED_SECTORS as u16).to_le_bytes());
    s[16] = FAT_COUNT as u8;
    // Root entry count and 16-bit totals are zero on FAT32 by definition.
    s[21] = 0xF8;
    s[24..26].copy_from_slice(&63u16.to_le_bytes());
    s[26..28].copy_from_slice(&255u16.to_le_bytes());
    s[28..32].copy_from_slice(&(hidden_sectors as u32).to_le_bytes());
    s[32..36].copy_from_slice(&(geo.partition_sectors as u32).to_le_bytes());
    s[36..40].copy_from_slice(&geo.fat_sectors.to_le_bytes());
    // Ext flags 0: both FATs mirrored. Version 0.0. Root at cluster 2.
    s[44..48].copy_from_slice(&2u32.to_le_bytes());
    s[48..50].copy_from_slice(&1u16.to_le_bytes());
    s[50..52].copy_from_slice(&(BACKUP_BOOT_SECTOR as u16).to_le_bytes());
    s[64] = 0x80;
    s[66] = 0x29;
    s[67..71].copy_from_slice(&volume_id.to_le_bytes());
    s[71..82].copy_from_slice(VOLUME_LABEL);
    s[82..90].copy_from_slice(b"FAT32   ");
    s[510] = 0x55;
    s[511] = 0xAA;
    s
}
