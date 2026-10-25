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
use super::plan_types::{Plan, PlanError};
use crate::hardware::block_device::{self, Places};

pub(super) fn read_plan() -> Result<Plan, VolumeError> {
    let capacity = block_device::capacity().map_err(VolumeError::Device)?;
    let mut sector = [0u8; 512];
    block_device::read_plan_sector(0, &mut sector).map_err(VolumeError::Device)?;
    let refused = |e| {
        crate::log::warn!("[DATA] disk plan at LBA {} refused: {:?}", block_device::plan_lba(), e);
        VolumeError::Plan(e)
    };
    let plan = parse_plan(&sector, capacity).map_err(refused)?;
    match block_device::places() {
        Some(p) if !in_data(&p, &plan) => Err(refused(PlanError::PastEnd)),
        _ => Ok(plan),
    }
}

/// A table that names a data partition holds the volume to it: a plan
/// pointing past it would have the volume written over another partition.
/// A live stick's plan keeps no volume, and a table naming no data
/// partition leaves the floor and the disk's end as the bounds.
fn in_data(p: &Places, plan: &Plan) -> bool {
    match p.data {
        Some(d) if plan.volume_sectors != 0 => {
            let end = plan.volume_base.checked_add(plan.volume_sectors);
            plan.volume_base >= d.first && end.is_some_and(|e| e <= d.end())
        }
        _ => true,
    }
}
