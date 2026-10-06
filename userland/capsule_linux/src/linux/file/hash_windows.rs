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

//! A file's BLAKE3 from windows read one at a time. Pure, so the proofs hold
//! it to the hash of the whole file.

use alloc::vec::Vec;

/// One window: what `read(offset, len)` returned.
type Window<E> = Result<Vec<u8>, E>;

/// The BLAKE3 of `size` bytes read `window` at a time. A read that comes back
/// empty before the end is the file ending early, said as `early`.
pub fn hash_windows<E>(
    size: u64,
    window: u32,
    early: E,
    mut read: impl FnMut(u64, u32) -> Window<E>,
) -> Result<[u8; 32], E> {
    let mut hasher = blake3::Hasher::new();
    let mut at = 0u64;
    while at < size {
        let want = (size - at).min(u64::from(window.max(1))) as u32;
        let got = read(at, want)?;
        if got.is_empty() {
            return Err(early);
        }
        hasher.update(&got);
        at += got.len() as u64;
    }
    Ok(*hasher.finalize().as_bytes())
}
