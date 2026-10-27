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

//! An image's digest, read from its source a bounded chunk at a time, with
//! no kernel dependencies so a host proof can run it.

use alloc::vec::Vec;

/// A Linux program with its interpreter is comfortably inside this.
pub(super) const MAX_ELF: usize = 128 << 20;

/// The most of an image the kernel holds at once.
pub(super) const CHUNK: usize = 1 << 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DigestError {
    /// Empty, or longer than MAX_ELF.
    Length,
    /// The heap could not spare a chunk.
    NoMemory,
    /// The source could not give a chunk.
    Unreadable,
}

/// The BLAKE3 digest of the `len` bytes `read` gives, `read(at, buf)` filling
/// `buf` from offset `at`. A length out of bounds is refused before anything
/// is read. `serve` runs before each read and each `unit` of other work. The
/// chunks go to one hasher in order, so the digest is that of the whole, and
/// a read that fails gives no digest at all.
pub(super) fn in_chunks<E>(
    len: u64,
    unit: usize,
    mut read: impl FnMut(u64, &mut [u8]) -> Result<(), E>,
    mut serve: impl FnMut(),
) -> Result<[u8; 32], DigestError> {
    if len == 0 || len > MAX_ELF as u64 {
        return Err(DigestError::Length);
    }
    let len = len as usize;
    let size = len.min(CHUNK);
    let mut bounce = Vec::new();
    if bounce.try_reserve_exact(size).is_err() {
        return Err(DigestError::NoMemory);
    }
    while bounce.len() < size {
        serve();
        bounce.resize(bounce.len() + (size - bounce.len()).min(unit), 0u8);
    }
    let mut hasher = blake3::Hasher::new();
    let mut done = 0;
    while done < len {
        let chunk = &mut bounce[..(len - done).min(CHUNK)];
        serve();
        read(done as u64, chunk).map_err(|_| DigestError::Unreadable)?;
        for piece in chunk.chunks(unit) {
            serve();
            hasher.update(piece);
        }
        done += chunk.len();
    }
    Ok(*hasher.finalize().as_bytes())
}
