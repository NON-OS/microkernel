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

//! CRC-64/XZ (ECMA-182, reflected, 0xC96C5795D7870F42): xz's default check.

const TABLE: [u64; 256] = table();

const fn table() -> [u64; 256] {
    let mut t = [0u64; 256];
    let mut n = 0;
    while n < 256 {
        let mut c = n as u64;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xC96C_5795_D787_0F42 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        t[n] = c;
        n += 1;
    }
    t
}

pub fn crc64(data: &[u8]) -> u64 {
    !data.iter().fold(!0u64, |c, &b| TABLE[((c ^ b as u64) & 0xFF) as usize] ^ (c >> 8))
}
