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

//! Where an installed disk's partitions must be for an update to rewrite
//! two of them and leave the rest: the store and the plan at the fixed
//! sectors the kernel reads (`nonos_disk_map`), the data partition from the
//! data floor up to the boot partition, and the boot partition one GiB or
//! more in whole MiBs, ending on the last MiB boundary before the backup
//! table, where `Layout::plan` puts it. An extent a sector off any of these
//! is not this writer's. The table's extents become the update's layout.

use nonos_disk_map::{DATA_FLOOR, PLAN_LBA, STORE_BASE_LBA};

use super::why::NotKept;
use crate::gpt::ARRAY_SECTORS;
use crate::layout::{Extent, Layout, ESP_SECTORS, MIB_SECTORS};

pub fn placed(total: u64, extents: &[Extent; 4]) -> Result<Layout, NotKept> {
    let [store, plan, data, esp] = *extents;
    let backup_header_lba = total.saturating_sub(1);
    let backup_array_lba = backup_header_lba.saturating_sub(ARRAY_SECTORS);
    let fixed = store == Extent::new(STORE_BASE_LBA, PLAN_LBA - STORE_BASE_LBA)
        && plan == Extent::new(PLAN_LBA, DATA_FLOOR - PLAN_LBA)
        && data.first == DATA_FLOOR
        && data.end() == esp.first;
    let at_end = esp.sectors >= ESP_SECTORS
        && esp.sectors.is_multiple_of(MIB_SECTORS)
        && esp.end() == backup_array_lba / MIB_SECTORS * MIB_SECTORS;
    if !(fixed && at_end) {
        return Err(NotKept::Moved);
    }
    Ok(Layout { total_sectors: total, store, plan, data, esp, backup_array_lba, backup_header_lba })
}
