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

//! Prior Poly1305 state and key setup: five 26-bit limbs in u32.

pub struct Poly1305 {
    pub h: [u32; 5],
    pub r: [u32; 5],
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s: [u8; 16],
    pub buffer: [u8; 16],
    pub buffer_len: usize,
}

impl Poly1305 {
    pub fn new(key: &[u8; 32]) -> Self {
        let mut r = [0u8; 16];
        r.copy_from_slice(&key[0..16]);
        r[3] &= 0x0f;
        r[7] &= 0x0f;
        r[11] &= 0x0f;
        r[15] &= 0x0f;
        r[4] &= 0xfc;
        r[8] &= 0xfc;
        r[12] &= 0xfc;
        let t0 = u32::from_le_bytes([r[0], r[1], r[2], r[3]]);
        let t1 = u32::from_le_bytes([r[4], r[5], r[6], r[7]]);
        let t2 = u32::from_le_bytes([r[8], r[9], r[10], r[11]]);
        let t3 = u32::from_le_bytes([r[12], r[13], r[14], r[15]]);
        let r0 = t0 & 0x3ffffff;
        let r1 = ((t0 >> 26) | (t1 << 6)) & 0x3ffffff;
        let r2 = ((t1 >> 20) | (t2 << 12)) & 0x3ffffff;
        let r3 = ((t2 >> 14) | (t3 << 18)) & 0x3ffffff;
        let r4 = t3 >> 8;
        let mut s = [0u8; 16];
        s.copy_from_slice(&key[16..32]);
        Self {
            h: [0; 5],
            r: [r0, r1, r2, r3, r4],
            s1: r1 * 5,
            s2: r2 * 5,
            s3: r3 * 5,
            s4: r4 * 5,
            s,
            buffer: [0u8; 16],
            buffer_len: 0,
        }
    }
}
