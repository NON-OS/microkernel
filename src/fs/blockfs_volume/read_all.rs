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

extern crate alloc;

use alloc::vec::Vec;

use super::error::VolumeError;
use super::state::VOLUME;
use crate::fs::blockfs;

/// The largest file read whole: the 64 MiB ceiling a program image has
/// without a certificate, the largest thing a whole read serves.
pub const WHOLE_READ_MAX: u64 = 64 * 1024 * 1024;

pub fn read_all(path: &[u8]) -> Result<Vec<u8>, VolumeError> {
    let guard = VOLUME.read();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    let lba = blockfs::resolve(&state.key, &state.mount, path).map_err(VolumeError::BlockFs)?;
    let node = blockfs::read_node(&state.key, lba).map_err(VolumeError::BlockFs)?;
    /*
     * node.size comes from a stored inode, so it is bounded before anything
     * is allocated for it. A file can now reach MAX_FILE_BYTES (over 6 GB),
     * far past what the kernel should hold at once, so a whole read has its
     * own ceiling and a larger file is refused by name; `read_at` reads any
     * file by range with a buffer of the caller's size.
     */
    if node.size > WHOLE_READ_MAX {
        return Err(VolumeError::TooLargeToReadWhole(node.size));
    }
    let mut out = alloc::vec![0u8; node.size as usize];
    let n = blockfs::read_file(&state.key, &node, &mut out).map_err(VolumeError::BlockFs)?;
    out.truncate(n);
    Ok(out)
}
