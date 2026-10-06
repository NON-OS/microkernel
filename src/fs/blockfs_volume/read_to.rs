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
//! Reading a range of a file through a stage the caller lends, handed on a
//! stage at a time. The file is found once and the volume's lock held for
//! the whole range, so its pieces share one pointer path and read-ahead run
//! as the pieces of `read_at` calls do, without a buffer the size of the
//! range. The caller wipes the stage.

use super::error::VolumeError;
use super::state::{READS, VOLUME};
use crate::fs::blockfs;

/// Up to `len` bytes of the file at `path` from `offset`, each stage of
/// them given to `hand` with how many went before it. Stops at the file's
/// end or when `hand` says false; returns the bytes `hand` took. `hand`
/// runs under the volume's read lock and must not fault or sleep.
pub fn read_to(
    path: &[u8],
    offset: u64,
    len: usize,
    stage: &mut [u8],
    mut hand: impl FnMut(usize, &[u8]) -> bool,
) -> Result<usize, VolumeError> {
    let guard = VOLUME.read();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    let lba = blockfs::resolve(&state.key, &state.mount, path).map_err(VolumeError::BlockFs)?;
    let node = blockfs::read_node(&state.key, lba).map_err(VolumeError::BlockFs)?;
    let mut done = 0usize;
    while done < len && !stage.is_empty() {
        let want = (len - done).min(stage.len());
        let at = offset.saturating_add(done as u64);
        let got = READS
            .read_at(&state.key, &state.mount, lba, &node, at, &mut stage[..want])
            .map_err(VolumeError::BlockFs)?;
        if got == 0 || !hand(done, &stage[..got]) {
            break;
        }
        done += got;
        if got < want {
            break;
        }
    }
    Ok(done)
}
