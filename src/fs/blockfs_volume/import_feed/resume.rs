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
 * Where a fed import starts on the volume: after its mark when one is kept
 * for this very file, from nothing otherwise, and only when the volume has
 * room for the rest.
 */

use super::super::error::VolumeError;
use super::super::state::VOLUME;
use super::error::StreamError;
use super::hash::PinHash;
use super::live::MARK_EVERY;
use super::mark::read;
use super::mark_codec::decode;
use crate::fs::blockfs::{self, BlockFsError, FileStream};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

/* Sectors kept free beyond the data: an index, a node, a record. */
const SPARE_SECTORS: u64 = 64;
/* Sectors each saved mark takes: its blocks, its index, the header. */
const MARK_SECTORS: u64 = 16;

pub(super) fn resume(
    name: &[u8],
    want: &[u8; 32],
    bytes: u64,
) -> Result<(PinHash, FileStream), StreamError> {
    let guard = VOLUME.read();
    let s = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    match blockfs::resolve(&s.key, &s.mount, name) {
        Err(BlockFsError::NotFound) => {}
        Ok(_) => return Err(VolumeError::NameTaken.into()),
        Err(e) => return Err(VolumeError::BlockFs(e).into()),
    }
    let kept = read(&s.key, &s.mount, name)?.and_then(|b| decode(&b, want, bytes));
    let (hash, stream) = kept.unwrap_or_else(|| (PinHash::new(), FileStream::new()));
    let data = (bytes - stream.size()).div_ceil(PLAIN_BLOCK_BYTES as u64);
    let pointers = data.div_ceil(PLAIN_BLOCK_BYTES as u64 / 8 - 1);
    let marks = (bytes - stream.size()).div_ceil(MARK_EVERY) * MARK_SECTORS;
    let free = blockfs::alloc_limit(&s.mount).saturating_sub(s.mount.superblock.free_lba);
    if data + pointers + marks + SPARE_SECTORS > free {
        return Err(StreamError::NoRoom);
    }
    Ok((hash, stream))
}
