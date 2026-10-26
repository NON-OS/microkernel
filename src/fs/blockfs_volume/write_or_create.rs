// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::error::VolumeError;
use super::state::VOLUME;
use crate::fs::blockfs::{self, BlockFsError, MODE_FILE};

pub fn write_or_create(path: &[u8], data: &[u8]) -> Result<(), VolumeError> {
    /*
     * One write hold: no other caller can make or remove the file between
     * the look and the write, and the import guard is asked under the hold
     * the write then uses, with the verb that names what happens.
     */
    let mut guard = VOLUME.write();
    let state = guard.as_mut().ok_or(VolumeError::NotMounted)?;
    let exists = match blockfs::resolve(&state.key, &state.mount, path) {
        Ok(_) => true,
        Err(BlockFsError::NotFound) => false,
        Err(e) => return Err(VolumeError::BlockFs(e)),
    };
    let what = if exists { "write" } else { "create" };
    super::import_guard::guard(&state.key, &state.mount, path, what)?;
    if !exists {
        blockfs::create_path(&state.key, &mut state.mount, path, MODE_FILE)
            .map_err(VolumeError::BlockFs)?;
    }
    blockfs::write_path(&state.key, &mut state.mount, path, data).map_err(VolumeError::BlockFs)
}
