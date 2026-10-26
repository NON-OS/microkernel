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

//! One candidate file, streamed in, hashed and linked only if it matched.

use super::error::VolumeError;
use super::hex::{as_str, hex32};
use super::import_guard::is_kept;
use super::import_record::record;
use super::import_stream::stream_in;
use super::imported::Imported;
use super::state::VOLUME;
use crate::fs::blockfs::{self, FileStream, MODE_FILE};

pub(super) fn import_from(
    name: &[u8],
    want: &[u8; 32],
    at: u64,
    bytes: u64,
) -> Result<Imported, VolumeError> {
    /* A record or a mark is the kernel's own, never a file brought in. */
    if is_kept(name) {
        return Err(VolumeError::ImportOnly);
    }
    let mut guard = VOLUME.write();
    let state = guard.as_mut().ok_or(VolumeError::NotMounted)?;
    let (key, mount) = (&state.key, &mut state.mount);
    let mut stream = FileStream::new();
    let got = stream_in(key, mount, &mut stream, at, bytes)?;
    if &got != want {
        let (g, w) = (hex32(&got), hex32(want));
        crate::log::warn!(
            "[DATA] import refused: the file at LBA {} hashes to {}, not the pinned {}",
            at,
            as_str(&g),
            as_str(&w)
        );
        return Err(VolumeError::DigestMismatch);
    }
    let lba = blockfs::create_path(key, mount, name, MODE_FILE).map_err(VolumeError::BlockFs)?;
    let mut node = blockfs::read_node(key, lba).map_err(VolumeError::BlockFs)?;
    stream.finish(key, mount, lba, &mut node).map_err(VolumeError::BlockFs)?;
    record(key, mount, name, want)?;
    let line = alloc::format!("[DATA] imported {} bytes, sha256 {}", bytes, as_str(&hex32(want)));
    super::say::say(&line);
    Ok(Imported { bytes, sha256: *want, fresh: true })
}
