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

//! CRC-32 as UEFI 2.10 names it for the GPT (as recalled, unverified): the
//! IEEE 802.3 polynomial 04C11DB7h reflected, all ones in and out. Written
//! here apart from the reader, held to the standard check value.

const TABLE: [u32; 256] = {
    let (mut t, mut i) = ([0u32; 256], 0);
    while i < 256 {
        let mut c = i as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 == 1 { (c >> 1) ^ 0x04C1_1DB7u32.reverse_bits() } else { c >> 1 };
            k += 1;
        }
        t[i] = c;
        i += 1;
    }
    t
};

pub fn crc32(data: &[u8]) -> u32 {
    !data.iter().fold(u32::MAX, |c, &b| TABLE[((c ^ u32::from(b)) & 0xFF) as usize] ^ (c >> 8))
}

#[test]
fn the_check_value_of_123456789_is_cbf43926() {
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
}
