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

//! ABI words: an address or integer right-aligned, and back strictly.

pub fn word(tail: &[u8]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[32 - tail.len()..].copy_from_slice(tail);
    w
}

/// The word as a u128, or None when its top 16 bytes are not zero.
pub fn u128_of(w: &[u8; 32]) -> Option<u128> {
    w[..16]
        .iter()
        .all(|b| *b == 0)
        .then(|| u128::from_be_bytes(w[16..].try_into().unwrap_or_default()))
}

/// The first word of a reply, as a u128.
pub fn first_u128(ret: Option<&alloc::vec::Vec<[u8; 32]>>) -> Option<u128> {
    ret.and_then(|r| r.first()).and_then(u128_of)
}
