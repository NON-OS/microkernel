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

//! Chunks: exactly CHUNK bytes each, the last zero-padded, and hashed
//! padded; n = ceil(content_len / CHUNK). Content of no bytes has no chunks.

pub const CHUNK: usize = 1024;

pub fn chunk_count(content_len: u64) -> u64 {
    content_len.div_ceil(CHUNK as u64)
}

/// Chunk `i` of `content`, padded; None past the last.
pub fn chunk(content: &[u8], i: u64) -> Option<[u8; CHUNK]> {
    let start = usize::try_from(i).ok()?.checked_mul(CHUNK)?;
    if start >= content.len() {
        return None;
    }
    let part = content.get(start..content.len().min(start.checked_add(CHUNK)?))?;
    let mut out = [0u8; CHUNK];
    out.get_mut(..part.len())?.copy_from_slice(part);
    Some(out)
}
