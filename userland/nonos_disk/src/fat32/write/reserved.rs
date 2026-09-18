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

//! The reserved sectors: boot sector and FSInfo at 0 and 1, their copies
//! at 6 and 7, zero everywhere else. Built as one block so a driver that
//! reads the whole reserved area finds nothing stale from a previous life
//! of the disk.

use alloc::vec::Vec;

use crate::fat32::bpb::{boot_sector, fs_info, BACKUP_BOOT_SECTOR};
use crate::fat32::geometry::{Geometry, FAT_COUNT, RESERVED_SECTORS};
use crate::sink::SECTOR_SIZE;

/// The whole reserved area as one buffer.
pub fn build_reserved(
    geo: &Geometry,
    first_lba: u64,
    used_clusters: u64,
    volume_id: u32,
) -> Vec<u8> {
    let mut area: Vec<u8> = alloc::vec![0u8; RESERVED_SECTORS as usize * SECTOR_SIZE];
    let boot = boot_sector(geo, first_lba, volume_id);
    let info = fs_info(geo, used_clusters);
    let backup = BACKUP_BOOT_SECTOR as usize * SECTOR_SIZE;
    area[..SECTOR_SIZE].copy_from_slice(&boot);
    area[SECTOR_SIZE..2 * SECTOR_SIZE].copy_from_slice(&info);
    area[backup..backup + SECTOR_SIZE].copy_from_slice(&boot);
    area[backup + SECTOR_SIZE..backup + 2 * SECTOR_SIZE].copy_from_slice(&info);
    area
}

/// First sector of the data area, where cluster 2 begins.
pub fn data_lba(first_lba: u64, geo: &Geometry) -> u64 {
    first_lba + RESERVED_SECTORS as u64 + FAT_COUNT as u64 * geo.fat_sectors as u64
}
