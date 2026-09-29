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

//! The CRC-32 of RFC 1952 section 8 (reflected, polynomial 0xEDB88320),
//! sixteen bytes per step with sixteen tables (slicing-by-16).

const T: [[u32; 256]; 16] = tables();

const fn tables() -> [[u32; 256]; 16] {
    let mut t = [[0u32; 256]; 16];
    let mut n = 0;
    while n < 256 {
        let mut c = n as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        t[0][n] = c;
        n += 1;
    }
    let mut n = 0;
    while n < 256 {
        let mut s = 1;
        while s < 16 {
            let prev = t[s - 1][n];
            t[s][n] = (prev >> 8) ^ t[0][(prev & 0xff) as usize];
            s += 1;
        }
        n += 1;
    }
    t
}

pub(super) fn crc32(data: &[u8]) -> u32 {
    let mut c = !0u32;
    let mut blocks = data.chunks_exact(16);
    for w in &mut blocks {
        let word = |i: usize| u32::from_le_bytes([w[i], w[i + 1], w[i + 2], w[i + 3]]);
        let mut x = 0;
        for (k, v) in [word(0) ^ c, word(4), word(8), word(12)].into_iter().enumerate() {
            for j in 0..4 {
                x ^= T[15 - 4 * k - j][(v >> (8 * j) & 0xff) as usize];
            }
        }
        c = x;
    }
    for &b in blocks.remainder() {
        c = T[0][((c ^ u32::from(b)) & 0xff) as usize] ^ (c >> 8);
    }
    !c
}
