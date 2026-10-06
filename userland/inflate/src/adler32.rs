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

//! The Adler-32 of RFC 1950 section 8.

const BASE: u32 = 65521;

/// Bytes summed before a reduction: the largest n with
/// 255 n (n + 1) / 2 + (n + 1) (BASE - 1) below 2^32.
const NMAX: usize = 5552;

pub(super) fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for block in data.chunks(NMAX) {
        for &x in block {
            a += u32::from(x);
            b += a;
        }
        a %= BASE;
        b %= BASE;
    }
    b << 16 | a
}
