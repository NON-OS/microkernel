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

//! The raw file the disk plan names, streamed into the volume and hashed.
//!
//! The bytes come from outside the machine, through a region anyone with the
//! disk can write, so they are hashed exactly as they are sealed: the digest
//! that decides whether the file is kept is the digest of what went into it.

use sha2::{Digest, Sha256};

use super::error::VolumeError;
use crate::fs::blockfs::{BlockFsMount, FileStream};

/// Sectors per read: every block driver's per-request ceiling.
const CHUNK_SECTORS: u64 = 64;

/// Seal `bytes` from device LBA `at` into `stream`; the SHA-256 of them.
pub(super) fn stream_in(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    stream: &mut FileStream,
    at: u64,
    bytes: u64,
) -> Result<[u8; 32], VolumeError> {
    let mut hash = Sha256::new();
    let mut buf = alloc::vec![0u8; (CHUNK_SECTORS * 512) as usize];
    let mut done = 0u64;
    while done < bytes {
        let take = (bytes - done).min(CHUNK_SECTORS * 512);
        let sectors = take.div_ceil(512);
        let lba = at + done / 512;
        let span = &mut buf[..(sectors * 512) as usize];
        crate::hardware::block_device::read(lba, span).map_err(VolumeError::Device)?;
        let part = &span[..take as usize];
        hash.update(part);
        stream.append(key, mount, part).map_err(VolumeError::BlockFs)?;
        super::say::progress(done, done + take, bytes);
        done += take;
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash.finalize());
    Ok(out)
}
