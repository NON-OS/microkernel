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
 * Where a streamed import's mark is kept: beside it, as `<name>.partial`,
 * sealed like every block on the volume.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::super::error::VolumeError;
use super::super::import_record::suffixed;
use crate::fs::blockfs::{self, BlockFsError, BlockFsMount, MODE_FILE};

const SUFFIX: &[u8] = b".partial";
/* Past any mark this writes, so a longer file reads as no mark at all. */
const MARK_MAX: usize = 8192;

type Key = [u8; 32];

pub(super) fn save(
    k: &Key,
    m: &mut BlockFsMount,
    name: &[u8],
    body: &[u8],
) -> Result<(), VolumeError> {
    let (path, len) = suffixed(name, SUFFIX)?;
    match blockfs::resolve(k, m, &path[..len]) {
        Err(BlockFsError::NotFound) => {
            blockfs::create_path(k, m, &path[..len], MODE_FILE).map(|_| ())
        }
        other => other.map(|_| ()),
    }
    .and_then(|()| blockfs::write_path(k, m, &path[..len], body))
    .map_err(VolumeError::BlockFs)
}

/* The mark kept for `name`, if there is one. */
pub(super) fn read(k: &Key, m: &BlockFsMount, name: &[u8]) -> Result<Option<Vec<u8>>, VolumeError> {
    let (path, len) = suffixed(name, SUFFIX)?;
    let mut buf = vec![0u8; MARK_MAX];
    match blockfs::read_path(k, m, &path[..len], &mut buf) {
        Ok(n) => Ok(Some(buf[..n].to_vec())),
        Err(BlockFsError::NotFound) => Ok(None),
        Err(e) => Err(VolumeError::BlockFs(e)),
    }
}

pub(super) fn forget(k: &Key, m: &BlockFsMount, name: &[u8]) -> Result<(), VolumeError> {
    let (path, len) = suffixed(name, SUFFIX)?;
    match blockfs::unlink_path(k, m, &path[..len]) {
        Ok(()) | Err(BlockFsError::NotFound) => Ok(()),
        Err(e) => Err(VolumeError::BlockFs(e)),
    }
}
