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

//! Little-endian reads at fixed offsets, each `None` past the end.

pub(crate) fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at.checked_add(2)?)?.try_into().ok()?))
}

pub(crate) fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at.checked_add(4)?)?.try_into().ok()?))
}

pub(crate) fn u64_at(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at.checked_add(8)?)?.try_into().ok()?))
}

pub(crate) fn b32_at(b: &[u8], at: usize) -> Option<[u8; 32]> {
    b.get(at..at.checked_add(32)?)?.try_into().ok()
}

/// Four words as 32 little-endian bytes, the way a record and an anchor
/// carry a field digest.
pub(crate) fn words_le(w: &[u64; 4]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (o, v) in out.chunks_exact_mut(8).zip(w) {
        o.copy_from_slice(&v.to_le_bytes());
    }
    out
}

/// The inverse of `words_le`, with no reduction: a caller that needs field
/// words checks them below p itself.
pub(crate) fn words_of_le(b: &[u8; 32]) -> [u64; 4] {
    core::array::from_fn(|i| {
        let mut w = [0u8; 8];
        w.copy_from_slice(&b[8 * i..8 * i + 8]);
        u64::from_le_bytes(w)
    })
}
