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

//! Reading what the checks need off a disk: sectors 0 to 33, the
//! protective MBR and the primary table, then the plan and the key header,
//! the two sectors at `PLAN_LBA`. Reads only, in parts an update leaves.

use nonos_disk_map::PLAN_LBA;

use super::classify::{classify, Keep};
use super::volume::plan_names_volume;
use super::why::NotKept;
use crate::boot_media::read::sectors;
use crate::gpt::FIRST_USABLE_LBA;
use crate::sink::{BlockSink, SECTOR_SIZE};

pub fn read_installed(disk: &mut dyn BlockSink) -> Keep {
    let Ok(total) = disk.capacity_sectors() else { return Keep::No(NotKept::Unread) };
    let head = sectors(disk, 0, FIRST_USABLE_LBA);
    match (head, sectors(disk, PLAN_LBA, 2)) {
        (Some(head), Some(two)) => {
            let (plan, key) = two.split_at(SECTOR_SIZE);
            classify(total, &head, plan, key)
        }
        _ => Keep::No(NotKept::Unread),
    }
}

/// Whether the disk's plan names a data volume, as an installed disk's
/// does and a live stick's never does: the one sector the installer reads
/// of the disk its own boot runs from.
pub fn names_a_volume(disk: &mut dyn BlockSink) -> bool {
    sectors(disk, PLAN_LBA, 1).is_some_and(|s| plan_names_volume(&s))
}
