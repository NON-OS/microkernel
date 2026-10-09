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
 * Ending a fed import. Put down, it keeps its mark for later. Finished, it
 * becomes the file only if every byte came, the SHA-256 of what was sealed
 * is the digest named at the start, and so is that of every byte read back
 * from the volume; on a mismatch the stream and its mark are discarded, and
 * its sealed blocks stay allocated, as the volume only allocates forward.
 */

use super::super::error::VolumeError;
use super::super::hex::{as_str, hex32};
use super::super::state::VOLUME;
use super::error::StreamError;
use super::link::link;
use super::live::{Live, LIVE};
use super::mark::forget;
use super::pause::pause;

/* End `pid`'s stream: put it down when `keep`, else finish it. The bytes it holds. */
pub fn stream_finish(pid: u32, keep: bool) -> Result<u64, StreamError> {
    let mut live = LIVE.lock();
    match live.as_ref() {
        Some(l) if l.pid != pid => return Err(StreamError::NotBegun),
        Some(l) if !keep && l.stream.size() != l.bytes => return Err(StreamError::Short),
        Some(_) => {}
        None => return Err(StreamError::NotBegun),
    }
    if keep {
        return pause(live.take());
    }
    let Some(Live { name, name_len, want, bytes, stream, hash, .. }) = live.take() else {
        return Err(StreamError::NotBegun);
    };
    drop(live);
    let name = &name[..name_len];
    let got = hash.finish();
    let mut guard = VOLUME.write();
    let s = guard.as_mut().ok_or(VolumeError::NotMounted)?;
    if got != want {
        let (g, w) = (hex32(&got), hex32(&want));
        crate::log::warn!(
            "[DATA] fed import refused: it hashes to {}, not the pinned {}",
            as_str(&g),
            as_str(&w)
        );
        forget(&s.key, &s.mount, name)?;
        return Err(VolumeError::DigestMismatch.into());
    }
    let linked = link(&s.key, &mut s.mount, name, stream, &want);
    if matches!(linked, Ok(()) | Err(VolumeError::DigestMismatch)) {
        forget(&s.key, &s.mount, name)?;
    }
    linked?;
    let line = alloc::format!("[DATA] imported {} bytes, sha256 {}", bytes, as_str(&hex32(&want)));
    super::super::say::say(&line);
    Ok(bytes)
}
