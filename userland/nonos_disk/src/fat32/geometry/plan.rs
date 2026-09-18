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

//! A FAT driver decides whether a volume is FAT12, FAT16 or FAT32 from the
//! cluster count alone, never from the label in the boot sector. Below 65525
//! clusters the firmware driver reads a FAT32 boot sector as FAT16 and the
//! disk does not boot. So the cluster is 4 KiB where the partition allows,
//! halving down to one sector for a small one, and a partition too small
//! for any of them is refused rather than written wrong.

use crate::sink::SECTOR_SIZE;

pub const MIN_FAT32_CLUSTERS: u64 = 65_525;
pub const MAX_FAT32_CLUSTERS: u64 = 0x0FFF_FFF5;
pub const RESERVED_SECTORS: u32 = 32;
pub const FAT_COUNT: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Geometry {
    pub partition_sectors: u64,
    pub sectors_per_cluster: u8,
    pub fat_sectors: u32,
    pub data_clusters: u64,
}

impl Geometry {
    pub fn cluster_bytes(&self) -> usize {
        self.sectors_per_cluster as usize * SECTOR_SIZE
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanError {
    TooSmall,
    TooLarge,
}

/// The largest cluster that keeps the volume FAT32, preferring 4 KiB.
pub fn plan(partition_sectors: u64) -> Result<Geometry, PlanError> {
    for &spc in &[8u8, 4, 2, 1] {
        let geo = fit(partition_sectors, spc);
        if geo.data_clusters > MAX_FAT32_CLUSTERS {
            return Err(PlanError::TooLarge);
        }
        if geo.data_clusters >= MIN_FAT32_CLUSTERS {
            return Ok(geo);
        }
    }
    Err(PlanError::TooSmall)
}

/// Each FAT holds four bytes per cluster plus two reserved entries, and the
/// data area is what two of them leave. Sizing the tables for the upper
/// bound and recounting once is exact: the table only shrinks.
fn fit(partition_sectors: u64, spc: u8) -> Geometry {
    let usable = partition_sectors.saturating_sub(RESERVED_SECTORS as u64);
    let fat_for = |clusters: u64| ((clusters + 2) * 4).div_ceil(SECTOR_SIZE as u64);
    let fat_sectors = fat_for(usable / spc as u64);
    let data_clusters = usable.saturating_sub(FAT_COUNT as u64 * fat_sectors) / spc as u64;
    let fat_sectors = fat_for(data_clusters) as u32;
    Geometry { partition_sectors, sectors_per_cluster: spc, fat_sectors, data_clusters }
}
