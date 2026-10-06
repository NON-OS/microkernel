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

//! Fitting the layout to a disk. Every quantity below is a sum or a
//! difference of whole MiBs and fixed sectors, so the only rounding is the
//! ESP's end, taken down to the last MiB boundary before the backup table.

use nonos_disk_map::{DATA_FLOOR, PLAN_LBA, STORE_BASE_LBA};

use super::extent::Extent;
use super::sizes::{DATA_MIN_SECTORS, ESP_SECTORS, MIB_SECTORS};
use crate::gpt::ARRAY_SECTORS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub total_sectors: u64,
    pub store: Extent,
    pub plan: Extent,
    pub data: Extent,
    pub esp: Extent,
    pub backup_array_lba: u64,
    pub backup_header_lba: u64,
}

impl Layout {
    /// The ESP for files of `esp_needed` sectors: one GiB, or whole MiBs
    /// past it for files that need more.
    pub fn esp_sectors_for(esp_needed: u64) -> u64 {
        ESP_SECTORS.max(esp_needed.div_ceil(MIB_SECTORS).saturating_mul(MIB_SECTORS))
    }

    /// The smallest disk for files of `esp_needed` sectors. It is
    /// `MIN_DISK_SECTORS` for every image that fits one GiB.
    pub fn needed(esp_needed: u64) -> u64 {
        let esp = Self::esp_sectors_for(esp_needed);
        (DATA_FLOOR + DATA_MIN_SECTORS + MIB_SECTORS).saturating_add(esp)
    }

    /// The layout of a disk of `total_sectors`, `None` below `needed`. The
    /// data volume takes everything from the data floor to the ESP.
    pub fn plan(total_sectors: u64, esp_needed: u64) -> Option<Layout> {
        if total_sectors < Self::needed(esp_needed) {
            return None;
        }
        let backup_header_lba = total_sectors - 1;
        let backup_array_lba = backup_header_lba - ARRAY_SECTORS;
        let esp_sectors = Self::esp_sectors_for(esp_needed);
        let esp_end = backup_array_lba / MIB_SECTORS * MIB_SECTORS;
        let esp = Extent::new(esp_end - esp_sectors, esp_sectors);
        Some(Layout {
            total_sectors,
            store: Extent::new(STORE_BASE_LBA, PLAN_LBA - STORE_BASE_LBA),
            plan: Extent::new(PLAN_LBA, DATA_FLOOR - PLAN_LBA),
            data: Extent::new(DATA_FLOOR, esp.first - DATA_FLOOR),
            esp,
            backup_array_lba,
            backup_header_lba,
        })
    }
}
