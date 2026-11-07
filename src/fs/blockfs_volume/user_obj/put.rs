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

//! Writing an object whole, and removing one.

use super::super::error::VolumeError;
use super::super::state::{VolumeState, VOLUME};
use super::path::{dir_of, path_of, ROOT};
use crate::fs::blockfs::{self, BlockFsError, MODE_DIR, MODE_FILE};

/// Writes object `id` as `data`, making it and its directories first when
/// they are not there. One write hold covers the look and the write.
/// A volume with no fresh blocks left reclaims the ones nothing reaches
/// and is asked once more (reclaim.rs).
pub fn user_put(id: u64, data: &[u8]) -> Result<(), VolumeError> {
    super::super::reclaim::with_room(|state| {
        made(state, ROOT, MODE_DIR)?;
        made(state, &dir_of(id), MODE_DIR)?;
        let path = path_of(id);
        made(state, &path, MODE_FILE)?;
        blockfs::write_path(&state.key, &mut state.mount, &path, data).map_err(VolumeError::BlockFs)
    })
}

/// Removes object `id`; NotFound when there is none.
pub fn user_remove(id: u64) -> Result<(), VolumeError> {
    let guard = VOLUME.write();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    blockfs::unlink_path(&state.key, &state.mount, &path_of(id)).map_err(VolumeError::BlockFs)
}

fn made(state: &mut VolumeState, path: &[u8], mode: u16) -> Result<(), VolumeError> {
    match blockfs::resolve(&state.key, &state.mount, path) {
        Ok(_) => Ok(()),
        Err(BlockFsError::NotFound) => {
            blockfs::create_path(&state.key, &mut state.mount, path, mode)
                .map(drop)
                .map_err(VolumeError::BlockFs)
        }
        Err(e) => Err(VolumeError::BlockFs(e)),
    }
}
