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

//! Whether the new release's boot files fit the boot partition an
//! installed disk already has: the FAT32 volume the writer lays over that
//! extent, holding the image's tree. An update never moves a partition, so
//! files that outgrow it leave the disk to a full install. The screen's
//! answer and the update's plan both come from `volume_for`.

use super::classify::Keep;
use super::why::NotKept;
use crate::fat32::{check_volume, place_with, plan as plan_geometry, Geometry, Placed};
use crate::image::NonosImage;
use crate::layout::Extent;

impl Keep {
    /// This answer for an update to `image`: a disk whose boot partition
    /// the image does not fit keeps nothing.
    pub fn for_image(self, image: &NonosImage<'_>) -> Keep {
        match self {
            Keep::Ready(i) | Keep::NeedsPrepare(i)
                if volume_for(&i.layout.esp, image).is_none() =>
            {
                Keep::No(NotKept::EspTooSmall)
            }
            other => other,
        }
    }
}

/// The volume over `esp` holding `image`, or `None` when it does not fit.
pub(crate) fn volume_for<'a>(
    esp: &Extent,
    image: &NonosImage<'a>,
) -> Option<(Geometry, Placed<'a>)> {
    let geometry = plan_geometry(esp.sectors).ok()?;
    let placed = place_with(&image.tree(), geometry.cluster_bytes());
    let fits = placed.data_clusters_needed <= geometry.data_clusters;
    (fits && check_volume(&placed).is_ok()).then_some((geometry, placed))
}
