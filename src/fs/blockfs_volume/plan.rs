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

//! Reading the disk plan, every range checked before it is believed.

use super::plan_types::{Plan, PlanError, DATA_FLOOR, MAGIC, MIN_VOLUME};

fn word(sector: &[u8; 512], at: usize) -> u64 {
    let mut b = [0u8; 8];
    b.copy_from_slice(&sector[at..at + 8]);
    u64::from_le_bytes(b)
}

/// Read and check the plan in `sector` for a disk of `capacity` sectors.
pub fn parse_plan(sector: &[u8; 512], capacity: u64) -> Result<Plan, PlanError> {
    if sector[..8] != MAGIC {
        return Err(PlanError::NoPlan);
    }
    let (base, sectors) = (word(sector, 8), word(sector, 16));
    let volume_end = within(base, sectors, capacity)?;
    if sectors < MIN_VOLUME {
        return Err(PlanError::VolumeTooSmall);
    }
    let (at, bytes) = (word(sector, 24), word(sector, 32));
    if bytes == 0 {
        return Ok(Plan { volume_base: base, volume_sectors: sectors, import: None });
    }
    let import_end = within(at, bytes.div_ceil(512), capacity)?;
    if at < volume_end && base < import_end {
        return Err(PlanError::Overlap);
    }
    Ok(Plan { volume_base: base, volume_sectors: sectors, import: Some((at, bytes)) })
}

/// The end of `[start, start + sectors)`, checked against the floor and the disk.
fn within(start: u64, sectors: u64, capacity: u64) -> Result<u64, PlanError> {
    if start < DATA_FLOOR {
        return Err(PlanError::BelowFloor);
    }
    let end = start.checked_add(sectors).ok_or(PlanError::PastEnd)?;
    if end > capacity {
        return Err(PlanError::PastEnd);
    }
    Ok(end)
}
