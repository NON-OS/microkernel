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

//! The disk plan, read from its sector and checked against the disk.

use super::error::VolumeError;
use super::plan::parse_plan;
use super::plan_types::{Plan, PLAN_LBA};

pub(super) fn read_plan() -> Result<Plan, VolumeError> {
    let capacity = crate::hardware::block_device::capacity().map_err(VolumeError::Device)?;
    let mut sector = [0u8; 512];
    crate::hardware::block_device::read(PLAN_LBA, &mut sector).map_err(VolumeError::Device)?;
    parse_plan(&sector, capacity).map_err(|e| {
        crate::log::warn!("[DATA] disk plan at LBA {} refused: {:?}", PLAN_LBA, e);
        VolumeError::Plan(e)
    })
}
