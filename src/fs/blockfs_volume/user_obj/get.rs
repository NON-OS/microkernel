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

//! An object's size, and a range of it.

use super::super::error::VolumeError;
use super::super::state::{READS, VOLUME};
use super::path::path_of;
use crate::fs::blockfs::{self, MODE_DIR};

/// Object `id`'s length; NotFound when there is none.
pub fn user_size(id: u64) -> Result<u64, VolumeError> {
    let guard = VOLUME.read();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    let lba =
        blockfs::resolve(&state.key, &state.mount, &path_of(id)).map_err(VolumeError::BlockFs)?;
    let node = blockfs::read_node(&state.key, lba).map_err(VolumeError::BlockFs)?;
    // Only the kernel makes anything under /u/, and never a directory at an object's path.
    if node.mode & MODE_DIR != 0 {
        return Err(VolumeError::BlockFs(blockfs::BlockFsError::NotFound));
    }
    Ok(node.size)
}

/// Reads object `id` from `offset` into `out`; 0 at its end.
pub fn user_read(id: u64, offset: u64, out: &mut [u8]) -> Result<usize, VolumeError> {
    let guard = VOLUME.read();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    let path = path_of(id);
    let lba = blockfs::resolve(&state.key, &state.mount, &path).map_err(VolumeError::BlockFs)?;
    let node = blockfs::read_node(&state.key, lba).map_err(VolumeError::BlockFs)?;
    READS.read_at(&state.key, &state.mount, lba, &node, offset, out).map_err(VolumeError::BlockFs)
}
