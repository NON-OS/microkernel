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

//! Prior Poly1305 block step: 26-bit limbs, u64 products.

use super::poly_new::Poly1305;

impl Poly1305 {
    pub fn block(&mut self, msg: &[u8], hibit: u32) {
        let t0 = u32::from_le_bytes([msg[0], msg[1], msg[2], msg[3]]);
        let t1 = u32::from_le_bytes([msg[4], msg[5], msg[6], msg[7]]);
        let t2 = u32::from_le_bytes([msg[8], msg[9], msg[10], msg[11]]);
        let t3 = u32::from_le_bytes([msg[12], msg[13], msg[14], msg[15]]);
        let h = &mut self.h;
        h[0] = h[0].wrapping_add(t0 & 0x3ffffff);
        h[1] = h[1].wrapping_add(((t0 >> 26) | (t1 << 6)) & 0x3ffffff);
        h[2] = h[2].wrapping_add(((t1 >> 20) | (t2 << 12)) & 0x3ffffff);
        h[3] = h[3].wrapping_add(((t2 >> 14) | (t3 << 18)) & 0x3ffffff);
        h[4] = h[4].wrapping_add((t3 >> 8) | hibit);
        let [h0, h1, h2, h3, h4] = h.map(u64::from);
        let [r0, r1, r2, r3, r4] = self.r.map(u64::from);
        let [s1, s2, s3, s4] = [self.s1, self.s2, self.s3, self.s4].map(u64::from);
        let d0 = h0 * r0 + h1 * s4 + h2 * s3 + h3 * s2 + h4 * s1;
        let d1 = h0 * r1 + h1 * r0 + h2 * s4 + h3 * s3 + h4 * s2;
        let d2 = h0 * r2 + h1 * r1 + h2 * r0 + h3 * s4 + h4 * s3;
        let d3 = h0 * r3 + h1 * r2 + h2 * r1 + h3 * r0 + h4 * s4;
        let d4 = h0 * r4 + h1 * r3 + h2 * r2 + h3 * r1 + h4 * r0;
        let mut c: u32;
        c = (d0 >> 26) as u32;
        h[0] = (d0 as u32) & 0x3ffffff;
        let d1 = d1 + c as u64;
        c = (d1 >> 26) as u32;
        h[1] = (d1 as u32) & 0x3ffffff;
        let d2 = d2 + c as u64;
        c = (d2 >> 26) as u32;
        h[2] = (d2 as u32) & 0x3ffffff;
        let d3 = d3 + c as u64;
        c = (d3 >> 26) as u32;
        h[3] = (d3 as u32) & 0x3ffffff;
        let d4 = d4 + c as u64;
        c = (d4 >> 26) as u32;
        h[4] = (d4 as u32) & 0x3ffffff;
        h[0] = h[0].wrapping_add(c * 5);
        c = h[0] >> 26;
        h[0] &= 0x3ffffff;
        h[1] = h[1].wrapping_add(c);
    }
}
