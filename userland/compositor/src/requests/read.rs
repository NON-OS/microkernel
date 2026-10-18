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

//! The one length bound every fixed-size request shares: a body is read
//! only when it is exactly the length its op names.

/// The body's first `N` little-endian u32 words when it is exactly `LEN`
/// bytes. A padding word is read like any other, so a decoder can require
/// it zero.
pub fn words<const N: usize, const LEN: usize>(body: &[u8]) -> Option<[u32; N]> {
    // An op whose words outrun its own length is a build error, not a
    // check at run time.
    const { assert!(N * 4 <= LEN) };
    if body.len() != LEN {
        return None;
    }
    let mut w = [0u32; N];
    for (i, v) in w.iter_mut().enumerate() {
        let b = &body[i * 4..i * 4 + 4];
        *v = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    }
    Some(w)
}

/// A u64 sent as two words, low first.
pub fn wide(lo: u32, hi: u32) -> u64 {
    lo as u64 | (hi as u64) << 32
}
