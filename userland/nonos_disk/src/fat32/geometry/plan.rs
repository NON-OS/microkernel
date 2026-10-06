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
/// data area is what two of them leave. A driver counts the clusters from
/// the table size in the boot sector, so the count here is the one that
/// size leaves, never one taken before the tables shrank: that left a
/// cluster on the volume this writer did not count, and every FSInfo free
/// count one short.
///
/// A table of `fat` sectors holds the clusters it leaves from some size on
/// and at no size below it, since a larger table both leaves fewer clusters
/// and has room for more. The smallest such size leaves the most clusters,
/// and is found by halving between one sector and the table for every
/// sector. Shrinking from the top by recounting instead stopped at the
/// first overshoot: partitions of 66591 to 66598 sectors, FAT32 with a
/// 513-sector table, kept 521 sectors and were refused as too small.
fn fit(partition_sectors: u64, spc: u8) -> Geometry {
    let usable = partition_sectors.saturating_sub(RESERVED_SECTORS as u64);
    let sector = SECTOR_SIZE as u64;
    let clusters_left =
        |fat: u64| usable.saturating_sub((FAT_COUNT as u64).saturating_mul(fat)) / spc as u64;
    let holds = |fat: u64| {
        clusters_left(fat).saturating_add(2).saturating_mul(4) <= fat.saturating_mul(sector)
    };
    let (mut lo, mut hi) = (1u64, (usable / spc as u64 + 2).saturating_mul(4).div_ceil(sector));
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if holds(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let data_clusters = clusters_left(lo);
    let fat_sectors = u32::try_from(lo).unwrap_or(u32::MAX);
    Geometry { partition_sectors, sectors_per_cluster: spc, fat_sectors, data_clusters }
}
