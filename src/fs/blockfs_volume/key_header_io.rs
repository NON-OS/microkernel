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
use crate::hardware::block_device;

/// How the volume is keyed, `None` for a disk without a key header.
pub(super) fn read_key_header() -> Result<Option<Keyed>, VolumeError> {
    let mut sector = [0u8; 512];
    block_device::read(KEY_LBA, &mut sector).map_err(VolumeError::Device)?;
    parse_key_header(&sector).map_err(|way| {
        crate::log::warn!("[DATA] key header names way {} this kernel does not know", way);
        VolumeError::UnknownKeying
    })
}

/// Record `keyed` and flush it to the medium before anything is sealed.
pub(super) fn write_key_header(keyed: &Keyed) -> Result<(), VolumeError> {
    block_device::write(KEY_LBA, &encode_key_header(keyed)).map_err(VolumeError::Device)?;
    block_device::flush().map_err(VolumeError::Device)
}
