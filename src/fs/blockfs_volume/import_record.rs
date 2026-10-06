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

//! The digest an import was verified against, kept beside it on the volume.
//!
//! The record is sealed like every block, so only this machine could have
//! written it, and a later boot trusts it instead of hashing the file again.

use super::error::VolumeError;
use super::state::VOLUME;
use crate::fs::blockfs::{self, BlockFsError, BlockFsMount, MODE_FILE};

const SUFFIX: &[u8] = b".sha256";
const NAME_MAX: usize = 96;

/// `name` with `suffix` after it: the name of a file kept beside it.
pub(super) fn suffixed(name: &[u8], suffix: &[u8]) -> Result<([u8; NAME_MAX], usize), VolumeError> {
    let len = name.len() + suffix.len();
    if name.is_empty() || len > NAME_MAX {
        return Err(VolumeError::BlockFs(BlockFsError::InvalidName));
    }
    let mut out = [0u8; NAME_MAX];
    out[..name.len()].copy_from_slice(name);
    out[name.len()..len].copy_from_slice(suffix);
    Ok((out, len))
}

/// The file's size when `name` was imported and verified as `want` before.
/// A name that holds another digest is taken, not overwritten.
pub(super) fn recorded(name: &[u8], want: &[u8; 32]) -> Result<Option<u64>, VolumeError> {
    let (rec, len) = suffixed(name, SUFFIX)?;
    let guard = VOLUME.read();
    let s = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    let mut got = [0u8; 32];
    match blockfs::read_path(&s.key, &s.mount, &rec[..len], &mut got) {
        Ok(32) if &got == want => {}
        Ok(_) => return Err(VolumeError::NameTaken),
        Err(BlockFsError::NotFound) => return Ok(None),
        Err(e) => return Err(VolumeError::BlockFs(e)),
    }
    let lba = blockfs::resolve(&s.key, &s.mount, name).map_err(VolumeError::BlockFs)?;
    let node = blockfs::read_node(&s.key, lba).map_err(VolumeError::BlockFs)?;
    Ok(Some(node.size))
}

/// Keep `want` beside `name`.
pub(super) fn record(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    name: &[u8],
    want: &[u8; 32],
) -> Result<(), VolumeError> {
    let (rec, len) = suffixed(name, SUFFIX)?;
    let lba =
        blockfs::create_path(key, mount, &rec[..len], MODE_FILE).map_err(VolumeError::BlockFs)?;
    let mut node = blockfs::read_node(key, lba).map_err(VolumeError::BlockFs)?;
    blockfs::write_file(key, mount, lba, &mut node, want).map_err(VolumeError::BlockFs)
}
