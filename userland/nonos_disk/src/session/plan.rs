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
//! geometry, where every run lands, the store, and the identifiers. Built
//! with no device access, so a plan that fails costs nothing and a plan that
//! succeeds is the whole install.

use super::ids::{Ids, ENTROPY_BYTES};
use crate::fat32::{check_volume, place_with, plan as plan_geometry, Geometry, Placed};
use crate::image::NonosImage;
use crate::layout::Layout;
use crate::sink::SECTOR_SIZE;
use crate::store::StoreImage;
use crate::writer::WriteError;

/// Slack past the image for the two FATs, the directories and the reserved
/// area: sixteen MiB is more than a one-GiB FAT32 volume's tables need.
const TABLE_SLACK_SECTORS: u64 = (16u64 << 20) / SECTOR_SIZE as u64;

pub struct Plan<'a> {
    pub layout: Layout,
    pub geometry: Geometry,
    pub placed: Placed<'a>,
    pub image: NonosImage<'a>,
    pub store: StoreImage,
    pub ids: Ids,
}

impl<'a> Plan<'a> {
    /// `entropy` seeds the disk GUID, the partition GUIDs and the volume id.
    pub fn new(
        total_sectors: u64,
        image: &NonosImage<'a>,
        store: StoreImage,
        entropy: [u8; ENTROPY_BYTES],
    ) -> Result<Plan<'a>, WriteError> {
        let needed = image.total_bytes().div_ceil(SECTOR_SIZE as u64) + TABLE_SLACK_SECTORS;
        let too_small =
            WriteError::DiskTooSmall { total_sectors, needed_sectors: Layout::needed(needed) };
        let layout = Layout::plan(total_sectors, needed).ok_or(too_small)?;
        let geometry =
            plan_geometry(layout.esp.sectors).map_err(|e| WriteError::Volume(e.into()))?;
        let placed = place_with(&image.tree(), geometry.cluster_bytes());
        if placed.data_clusters_needed > geometry.data_clusters {
            return Err(WriteError::Volume(crate::fat32::PlanError::TooSmall.into()));
        }
        check_volume(&placed)?;
        let ids = Ids::from_entropy(&entropy);
        Ok(Plan { layout, geometry, placed, image: *image, store, ids })
    }
}
