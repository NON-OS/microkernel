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

//! Digests as text.

// A digest as lowercase hex. The buffer the caller owns decides how much of it
// lands: a 32-byte hash is 64 columns, which no card here is wide enough to
// hold, so a short buffer truncates on a byte boundary and the caller says so
// rather than this returning a half-written pair.
pub fn hex_bytes<'a>(src: &[u8], dst: &'a mut [u8]) -> &'a [u8] {
    let mut n = 0;
    for byte in src {
        if n + 2 > dst.len() {
            break;
        }
        dst[n] = nibble(byte >> 4);
        dst[n + 1] = nibble(byte & 0xf);
        n += 2;
    }
    &dst[..n]
}

fn nibble(v: u8) -> u8 {
    if v < 10 {
        b'0' + v
    } else {
        b'a' + v - 10
    }
}
