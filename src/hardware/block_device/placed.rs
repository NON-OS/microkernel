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

//! The store, the plan and the key header, at the sectors the kept disk's
//! table gives them (`gpt::window`). Until a disk is kept they are asked at the fixed sectors, where the
//! loader's copy of a stick holds them. The read that keeps a disk may find
//! its table placing them elsewhere; such a read is made again there, so no
//! answer comes from the wrong sectors.

use super::gpt::{plan_at, store_at, Places, FIXED};
use super::kept::places;
use super::{read, selected, write, BlockDeviceError};

/// Where a sector lies on a disk with these places, if anywhere.
type Lba = Option<u64>;

fn now() -> Places {
    places().unwrap_or(FIXED)
}

fn read_placed(at: impl Fn(&Places) -> Lba, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    let lba = at(&now()).ok_or(BlockDeviceError::OutOfRange)?;
    read(lba, out)?;
    match at(&now()) {
        Some(again) if again != lba => read(again, out),
        Some(_) => Ok(()),
        None => Err(BlockDeviceError::OutOfRange),
    }
}

/// Writes go to a kept disk only, so it is kept first and its places used.
fn write_placed(at: impl Fn(&Places) -> Lba, data: &[u8]) -> Result<(), BlockDeviceError> {
    selected()?;
    write(at(&now()).ok_or(BlockDeviceError::OutOfRange)?, data)
}

/// Store sectors from `lba`, as vfs numbers them.
pub fn read_store(lba: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    let n = out.len().div_ceil(512) as u64;
    read_placed(|p| store_at(p, lba, n), out)
}

pub fn write_store(lba: u64, data: &[u8]) -> Result<(), BlockDeviceError> {
    let n = data.len().div_ceil(512) as u64;
    write_placed(|p| store_at(p, lba, n), data)
}

/// Sector `index` of the plan partition: 0 the plan, 1 the key header.
pub fn read_plan_sector(index: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    read_placed(|p| plan_at(p, index), out)
}

pub fn write_plan_sector(index: u64, data: &[u8]) -> Result<(), BlockDeviceError> {
    write_placed(|p| plan_at(p, index), data)
}

/// Where the plan is asked now, for the log.
pub fn plan_lba() -> u64 {
    now().plan.first
}
