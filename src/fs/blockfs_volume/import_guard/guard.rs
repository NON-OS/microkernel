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
 * What a write from outside the import path may not touch: a record, a
 * mark, or a file that has a record. Checked under the same lock the write
 * then takes, so no import can land between the check and the write, and a
 * file written through the VFS never passes for one the kernel verified.
 */

use super::super::error::VolumeError;
use super::name::{is_kept, record_of};
use crate::fs::blockfs::{self, BlockFsError, BlockFsMount};

/* Refuse `what` on `path` when the file there is the import path's. */
pub(in super::super) fn guard(
    key: &[u8; 32],
    mount: &BlockFsMount,
    path: &[u8],
    what: &str,
) -> Result<(), VolumeError> {
    let imported = is_kept(path)
        || match blockfs::resolve(key, mount, &record_of(path)) {
            Ok(_) => true,
            Err(BlockFsError::NotFound) => false,
            Err(e) => return Err(VolumeError::BlockFs(e)),
        };
    if !imported {
        return Ok(());
    }
    let name = core::str::from_utf8(path).unwrap_or("a name");
    crate::log::warn!("[DATA] {} of {} refused: only an import writes it", what, name);
    Err(VolumeError::ImportOnly)
}
