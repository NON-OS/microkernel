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
 * The SHA-256 of a streamed file as the volume holds it, read back from its
 * sealed blocks. A stream taken up from a mark hashed its first bytes in an
 * earlier boot; what is recorded as verified is the digest of every byte
 * the volume now gives back, hashed here, never a state carried over.
 */

use alloc::vec;
use sha2::{Digest, Sha256};

use super::super::error::VolumeError;
use crate::fs::blockfs::{self, BlockFsError};

/* Bytes read back and hashed between two answers to TLB shootdowns. */
const PIECE: usize = 1 << 20;

/* The digest of the `bytes` long file whose node is at `lba`. */
pub(super) fn reread(key: &[u8; 32], lba: u64, bytes: u64) -> Result<[u8; 32], VolumeError> {
    let bs = VolumeError::BlockFs;
    let node = blockfs::read_node(key, lba).map_err(bs)?;
    if node.size != bytes {
        return Err(bs(BlockFsError::InvalidRecord));
    }
    let (mut hash, mut buf, mut at) = (Sha256::new(), vec![0u8; PIECE], 0u64);
    while at < bytes {
        crate::smp::serve_shootdowns();
        let n = blockfs::read_file_at(key, &node, at, &mut buf).map_err(bs)?;
        if n == 0 {
            return Err(bs(BlockFsError::InvalidRecord));
        }
        hash.update(&buf[..n]);
        at += n as u64;
    }
    Ok(hash.finalize().into())
}
