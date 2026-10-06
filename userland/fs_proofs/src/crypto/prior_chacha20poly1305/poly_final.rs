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

//! Prior Poly1305 final reduction and pad addition.

use super::poly_new::Poly1305;

impl Poly1305 {
    pub fn finalize(&mut self) -> [u8; 16] {
        if self.buffer_len > 0 {
            let mut block = [0u8; 16];
            block[..self.buffer_len].copy_from_slice(&self.buffer[..self.buffer_len]);
            block[self.buffer_len] = 1;
            self.block(&block, 0);
        }
        let h = &mut self.h;
        for i in [1, 2, 3, 4, 0] {
            let c = h[i] >> 26;
            h[i] &= 0x3ffffff;
            let (next, add) = if i == 4 { (0, c * 5) } else { ((i + 1) % 5, c) };
            h[next] = h[next].wrapping_add(add);
        }
        let mut g = [0u32; 5];
        let mut c = 5;
        for i in 0..4 {
            g[i] = h[i].wrapping_add(c);
            c = g[i] >> 26;
            g[i] &= 0x3ffffff;
        }
        g[4] = h[4].wrapping_add(c).wrapping_sub(1 << 26);
        let mask = !((g[4] >> 31).wrapping_sub(1));
        for i in 0..5 {
            h[i] = (h[i] & mask) | (g[i] & !mask);
        }
        let f = [h[0] | (h[1] << 26), (h[1] >> 6) | (h[2] << 20), (h[2] >> 12) | (h[3] << 14)];
        let f = [f[0], f[1], f[2], (h[3] >> 18) | (h[4] << 8)];
        let mut tag = [0u8; 16];
        let mut carry = 0u16;
        for i in 0..16 {
            let v = (f[i / 4].to_le_bytes()[i % 4]) as u16 + self.s[i] as u16 + carry;
            tag[i] = v as u8;
            carry = v >> 8;
        }
        tag
    }
}
