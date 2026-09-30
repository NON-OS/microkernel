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
 * Feeding the next bytes of a stream: sealed into the volume and hashed
 * exactly as they are sealed, with a mark saved every `MARK_EVERY` bytes and
 * at the last byte.
 */

use super::super::error::VolumeError;
use super::super::state::VOLUME;
use super::error::StreamError;
use super::live::{Live, LIVE, MARK_EVERY};
use super::pause::mark;
use crate::fs::blockfs::BlockFsError;
use crate::fs::cryptoblock::flush_held;

/* Seal `chunk` after what `pid`'s stream holds. How far it has come. */
pub fn stream_write(pid: u32, chunk: &[u8]) -> Result<u64, StreamError> {
    let mut live = LIVE.lock();
    let l = match live.as_mut() {
        Some(l) if l.pid == pid => l,
        _ => return Err(StreamError::NotBegun),
    };
    if l.stream.size().saturating_add(chunk.len() as u64) > l.bytes {
        return Err(StreamError::Overrun);
    }
    let fed = feed(l, chunk);
    /*
     * A refusal part way leaves the writer holding part of the chunk, so the
     * stream in memory is dropped; the mark on the volume is whole, and the
     * next begin goes on from it.
     */
    if fed.is_err() {
        *live = None;
    }
    fed
}

fn feed(l: &mut Live, chunk: &[u8]) -> Result<u64, StreamError> {
    let before = l.stream.size();
    {
        let mut guard = VOLUME.write();
        let s = guard.as_mut().ok_or(VolumeError::NotMounted)?;
        l.stream.append(&s.key, &mut s.mount, chunk).map_err(VolumeError::BlockFs)?;
        /*
         * Sent before the lock is let go: sectors left held could be sent,
         * and lost, by another caller that would not know they were ours.
         */
        flush_held().map_err(|e| VolumeError::BlockFs(BlockFsError::CryptoBlock(e)))?;
    }
    l.hash.update(chunk);
    let after = l.stream.size();
    if after - l.marked >= MARK_EVERY || after == l.bytes {
        mark(l)?;
    }
    super::super::say::progress(before, after, l.bytes);
    Ok(after)
}
