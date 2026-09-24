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

//! Everything decided before the first sector: the disk layout, the volume
//! geometry, where every run lands, and the identifiers. Built from the
//! device's size and the image, with no device access, so a plan that
//! fails costs nothing and a plan that succeeds is the whole install.

use crate::fat32::{place_with, plan as plan_geometry, Geometry, Placed};
use crate::gpt::Layout;
use crate::guid::Guid;
use crate::image::NonosImage;
use crate::sink::SECTOR_SIZE;
use crate::writer::WriteError;

/// Slack past the image for the two FATs, the directories and the reserved
/// area: sixteen MiB is more than a one-GiB FAT32 volume's tables need.
const TABLE_SLACK_SECTORS: u64 = (16u64 << 20) / SECTOR_SIZE as u64;

pub struct Plan<'a> {
    pub layout: Layout,
    pub geometry: Geometry,
    pub placed: Placed<'a>,
    pub disk_guid: Guid,
    pub partition_guid: Guid,
    pub volume_id: u32,
}

impl<'a> Plan<'a> {
    /// `entropy` seeds the disk GUID, the partition GUID and the volume id.
    pub fn new(
        total_sectors: u64,
        image: &NonosImage<'a>,
        entropy: [u8; 36],
    ) -> Result<Plan<'a>, WriteError> {
        let needed = image.total_bytes().div_ceil(SECTOR_SIZE as u64) + TABLE_SLACK_SECTORS;
        let layout = Layout::plan(total_sectors, needed)
            .ok_or(WriteError::DiskTooSmall { total_sectors, needed_sectors: needed })?;
        let geometry =
            plan_geometry(layout.esp_sectors()).map_err(|e| WriteError::Volume(e.into()))?;
        let placed = place_with(&image.tree(), geometry.cluster_bytes());
        if placed.data_clusters_needed > geometry.data_clusters {
            return Err(WriteError::Volume(crate::fat32::PlanError::TooSmall.into()));
        }
        let (mut disk, mut part) = ([0u8; 16], [0u8; 16]);
        disk.copy_from_slice(&entropy[..16]);
        part.copy_from_slice(&entropy[16..32]);
        let volume_id = u32::from_le_bytes([entropy[32], entropy[33], entropy[34], entropy[35]]);
        Ok(Plan {
            layout,
            geometry,
            placed,
            disk_guid: Guid::from_random(disk),
            partition_guid: Guid::from_random(part),
            volume_id,
        })
    }
}
