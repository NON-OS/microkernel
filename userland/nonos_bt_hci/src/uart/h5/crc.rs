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

//! The Data Integrity Check (Vol 4, Part D, its section of that name): a
//! 16-bit CRC with the CCITT generator x^16 + x^12 + x^5 + 1, started
//! at 0xFFFF, over the header and payload. Each octet's bits go through
//! least significant first, as the UART sends them, so the register runs
//! reflected. The result is then bit-reversed and sent most significant
//! octet first.

pub fn crc(data: &[u8]) -> u16 {
    let mut c: u16 = 0xffff;
    for &b in data {
        c ^= u16::from(b);
        for _ in 0..8 {
            c = if c & 1 != 0 { c >> 1 ^ 0x8408 } else { c >> 1 };
        }
    }
    c.reverse_bits()
}
