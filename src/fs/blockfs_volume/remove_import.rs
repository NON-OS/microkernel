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


/*
 * Taking an imported file away, with the record the import kept beside it:
 * the one change to an import's files that is not an import, so it is the
 * import path's too. Refused only while a stream is feeding the name; a
 * download put down part way is taken away too, its mark with it. The file
 * goes first: a record left by a removal cut short names a file that is
 * gone, and the next import of the name takes the record away.
 */

use super::error::VolumeError;
use super::import_feed::without_stream;
use super::import_guard::is_kept;
use super::import_record::suffixed;
use super::state::VOLUME;
use crate::fs::blockfs::{self, BlockFsError};

pub fn remove_import(name: &[u8]) -> Result<(), VolumeError> {
    if is_kept(name) {
        return Err(VolumeError::ImportOnly);
    }
    let (mark, mark_len) = suffixed(name, b".partial")?;
    let (record, record_len) = suffixed(name, b".sha256")?;
    without_stream(name, || {
        let guard = VOLUME.write();
        let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
        let (key, mount) = (&state.key, &state.mount);
        let gone = |path: &[u8]| match blockfs::unlink_path(key, mount, path) {
            Ok(()) => Ok(true),
            Err(BlockFsError::NotFound) => Ok(false),
            Err(e) => Err(VolumeError::BlockFs(e)),
        };
        let file = gone(name)?;
        let record = gone(&record[..record_len])?;
        let paused = gone(&mark[..mark_len])?;
        if file || record || paused {
            Ok(())
        } else {
            Err(VolumeError::BlockFs(BlockFsError::NotFound))
        }
    })
}

/* A record whose file is gone, as a removal cut short leaves: taken away. */
pub(super) fn forget_record(name: &[u8]) -> Result<(), VolumeError> {
    let (record, len) = suffixed(name, b".sha256")?;
    let guard = VOLUME.write();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    match blockfs::unlink_path(&state.key, &state.mount, &record[..len]) {
        Ok(()) | Err(BlockFsError::NotFound) => Ok(()),
        Err(e) => Err(VolumeError::BlockFs(e)),
    }
}
