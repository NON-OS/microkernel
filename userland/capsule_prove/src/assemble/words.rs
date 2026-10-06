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

//! Field words as bytes: little-endian u64s, each below p. A word is canonical
//! exactly when `words_of`, which reduces, hands it back unchanged, so the
//! check is the reduction's own round trip and restates no modulus.

use alloc::vec::Vec;

use nonos_device_attest::words_of;

/// Whether every word of `b` is below p, so `words_of(b)` is `b` exactly.
pub fn canonical(b: &[u8; 32]) -> bool {
    let w = words_of(b);
    w.iter()
        .zip(b.chunks_exact(8))
        .all(|(f, c)| <[u8; 8]>::try_from(c).is_ok_and(|c| f.to_u64() == u64::from_le_bytes(c)))
}

/// Append each word as eight little-endian bytes.
pub fn put(out: &mut Vec<u8>, words: impl IntoIterator<Item = u64>) {
    for w in words {
        out.extend_from_slice(&w.to_le_bytes());
    }
}

/// The 32 bytes at `at`, when they are there and canonical.
pub fn take(b: &[u8], at: usize) -> Option<[u8; 32]> {
    let w: [u8; 32] = b.get(at..at.checked_add(32)?)?.try_into().ok()?;
    canonical(&w).then_some(w)
}

/// The little-endian integer of `N` bytes at `at`, when they are there.
pub fn le<const N: usize>(b: &[u8], at: usize) -> Option<[u8; N]> {
    b.get(at..at.checked_add(N)?)?.try_into().ok()
}
