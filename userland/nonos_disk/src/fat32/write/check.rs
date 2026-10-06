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

//! Everything about a placed tree that would stop its volume being written,
//! found before the first sector goes out: a directory whose names no slot
//! can spell, or spells twice, and a file past what a slot's 32-bit size
//! field holds. The queue encodes each directory again as it writes it and
//! took an encoding that failed as an empty directory, so a name that could
//! not be spelt became a volume missing the files under it.

use crate::fat32::dir::encode;
use crate::fat32::tree::{Content, Placed};

use super::error::WriteVolumeError;

pub fn check(placed: &Placed<'_>) -> Result<(), WriteVolumeError> {
    for (i, run) in placed.runs.iter().enumerate() {
        match &run.content {
            Content::Dir(_) => {
                encode(i, &placed.runs)?;
            }
            Content::File(data) if !fits_a_slot(data.len()) => {
                return Err(WriteVolumeError::FileTooLarge);
            }
            Content::File(_) => {}
        }
    }
    Ok(())
}

/// Whether a file of `len` bytes can be described by one directory slot,
/// whose size field is 32 bits: FAT32's four GiB less a byte.
pub fn fits_a_slot(len: usize) -> bool {
    u32::try_from(len).is_ok()
}
