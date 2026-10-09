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
 * A finished stream linked under its name and recorded as verified, but only
 * when what the volume gives back hashes to the pin. Anything that fails
 * after the name is taken takes the name and its record away again, so no
 * empty or unverified file blocks the name, and the mark, left in place,
 * lets the next finish link it again.
 */

use super::super::error::VolumeError;
use super::super::hex::{as_str, hex32};
use super::super::import_record::{record, suffixed};
use super::reread::reread;
use crate::fs::blockfs::{self, BlockFsMount, FileStream, MODE_FILE};

type Key = [u8; 32];

pub(super) fn link(
    k: &Key,
    m: &mut BlockFsMount,
    name: &[u8],
    stream: FileStream,
    want: &[u8; 32],
) -> Result<(), VolumeError> {
    let bs = VolumeError::BlockFs;
    let lba = blockfs::create_path(k, m, name, MODE_FILE).map_err(bs)?;
    let linked = seal(k, m, lba, stream, want).and_then(|()| record(k, m, name, want));
    if linked.is_err() {
        /* Best effort: a failed unlink leaves what the next begin refuses. */
        blockfs::unlink_path(k, m, name).ok();
        let (rec, len) = suffixed(name, b".sha256")?;
        blockfs::unlink_path(k, m, &rec[..len]).ok();
    }
    linked
}

/* Seal the stream's last blocks into the file at `lba`, then hold it to `want`. */
fn seal(
    k: &Key,
    m: &mut BlockFsMount,
    lba: u64,
    stream: FileStream,
    want: &[u8; 32],
) -> Result<(), VolumeError> {
    let bytes = stream.size();
    let mut node = blockfs::read_node(k, lba).map_err(VolumeError::BlockFs)?;
    stream.finish(k, m, lba, &mut node).map_err(VolumeError::BlockFs)?;
    let got = reread(k, lba, bytes)?;
    if &got != want {
        let (g, w) = (hex32(&got), hex32(want));
        crate::log::warn!(
            "[DATA] fed import refused: read back it hashes to {}, not the pinned {}",
            as_str(&g),
            as_str(&w)
        );
        return Err(VolumeError::DigestMismatch);
    }
    Ok(())
}
