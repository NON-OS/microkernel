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

//! The plan for an update: the installed disk's own layout and identifiers
//! in place of a fresh layout and fresh ones, the boot partition's volume
//! laid over the extent it already has, and the new store. Built with no
//! device access, as an install's plan is, from what `read_installed` read.

use super::ids::{Ids, ENTROPY_BYTES};
use super::plan::Plan;
use crate::fat32::PlanError;
use crate::image::NonosImage;
use crate::keep::{volume_for, Installed};
use crate::store::StoreImage;
use crate::writer::WriteError;

impl<'a> Plan<'a> {
    /// `entropy` seeds the boot partition's FAT volume id alone: the disk
    /// and its partitions keep the GUIDs their table gives them, since the
    /// table is not written.
    pub fn keep(
        installed: &Installed,
        image: &NonosImage<'a>,
        store: StoreImage,
        entropy: [u8; ENTROPY_BYTES],
    ) -> Result<Plan<'a>, WriteError> {
        let layout = installed.layout;
        let too_small = WriteError::Volume(PlanError::TooSmall.into());
        let (geometry, placed) = volume_for(&layout.esp, image).ok_or(too_small)?;
        let fresh = Ids::from_entropy(&entropy);
        let ids = Ids { disk: installed.disk, partitions: installed.partitions, ..fresh };
        Ok(Plan { layout, geometry, placed, image: *image, store, ids, keep: true })
    }
}
