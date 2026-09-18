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

//! Where the table and the one partition sit on a disk of a given size.
//!
//! The ESP starts one MiB in, the alignment every current disk wants, and
//! is one GiB when the disk allows, which leaves the image room to grow
//! across releases without a repartition. Whatever lies past it stays
//! unallocated on purpose: an installed NONOS keeps no state on disk, so
//! there is nothing to put there, and a partition that exists invites
//! something to be.

use crate::sink::SECTOR_SIZE;

pub const ENTRY_COUNT: u32 = 128;
pub const ENTRY_SIZE: u32 = 128;
pub const ARRAY_SECTORS: u64 = (ENTRY_COUNT * ENTRY_SIZE) as u64 / SECTOR_SIZE as u64;
pub const ESP_FIRST_LBA: u64 = 2048;
const ESP_PREFERRED_SECTORS: u64 = (1u64 << 30) / SECTOR_SIZE as u64;
const MIB_SECTORS: u64 = (1u64 << 20) / SECTOR_SIZE as u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub total_sectors: u64,
    pub esp_first_lba: u64,
    pub esp_last_lba: u64,
    pub backup_array_lba: u64,
    pub backup_header_lba: u64,
}

impl Layout {
    /// Fit the table and the partition to `total_sectors`. The partition is
    /// at least `needed_sectors` and at most the preferred size, whole MiBs
    /// so its end is aligned like its start. `None` when the disk cannot
    /// hold the table plus the minimum.
    pub fn plan(total_sectors: u64, needed_sectors: u64) -> Option<Layout> {
        let backup_header_lba = total_sectors.checked_sub(1)?;
        let backup_array_lba = backup_header_lba.checked_sub(ARRAY_SECTORS)?;
        let last_usable = backup_array_lba.checked_sub(1)?;
        let available = last_usable.checked_sub(ESP_FIRST_LBA)? + 1;
        let want = available.min(ESP_PREFERRED_SECTORS.max(needed_sectors));
        let want = want / MIB_SECTORS * MIB_SECTORS;
        if want < needed_sectors || want == 0 {
            return None;
        }
        Some(Layout {
            total_sectors,
            esp_first_lba: ESP_FIRST_LBA,
            esp_last_lba: ESP_FIRST_LBA + want - 1,
            backup_array_lba,
            backup_header_lba,
        })
    }

    pub fn esp_sectors(&self) -> u64 {
        self.esp_last_lba - self.esp_first_lba + 1
    }
}
