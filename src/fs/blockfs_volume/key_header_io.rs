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

//! Reading and writing the key header on the disk the block layer chose.

use super::error::VolumeError;
use super::key_header::{parse_key_header, Keyed, KEY_LBA};
use super::key_seal::encode_key_header;
use super::plan_types::PLAN_LBA;
use crate::hardware::block_device::{self, BlockDeviceError};

/// The key header's sector in the plan partition: the one after the plan.
const KEY_INDEX: u64 = KEY_LBA - PLAN_LBA;

/// How the volume is keyed, `None` for a disk without a key header.
pub(super) fn read_key_header() -> Result<Option<Keyed>, VolumeError> {
    let _io = crate::fs::cryptoblock::volume_scope().map_err(VolumeError::Device)?;
    let mut sector = [0u8; 512];
    block_device::read_plan_sector(KEY_INDEX, &mut sector).map_err(VolumeError::Device)?;
    parse_key_header(&sector).map_err(|way| {
        crate::log::warn!("[DATA] key header names way {} this kernel does not know", way);
        VolumeError::UnknownKeying
    })
}

/// Record `keyed` and flush it to the medium before anything is sealed. On a
/// 4096-byte-LBA disk the key sector shares its block with the plan's, and
/// the write rewrites both: the plan must read back as it was, or nothing is
/// formatted.
pub(super) fn write_key_header(keyed: &Keyed) -> Result<(), VolumeError> {
    let _io = crate::fs::cryptoblock::volume_scope().map_err(VolumeError::Device)?;
    let before = plan_sector()?;
    block_device::write_plan_sector(KEY_INDEX, &encode_key_header(keyed))
        .map_err(VolumeError::Device)?;
    block_device::flush().map_err(VolumeError::Device)?;
    if plan_sector()? != before {
        crate::log::warn!("[DATA] the disk plan changed under the key header's write");
        return Err(VolumeError::Device(BlockDeviceError::DeviceFailure));
    }
    Ok(())
}

/// The plan's own sector, the first of the plan partition.
fn plan_sector() -> Result<[u8; 512], VolumeError> {
    let mut sector = [0u8; 512];
    block_device::read_plan_sector(0, &mut sector).map_err(VolumeError::Device)?;
    Ok(sector)
}
