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

use super::core::{HIBIT, M42, M44};
use super::types::Poly1305;

impl Poly1305 {
    pub(crate) fn update(&mut self, mut data: &[u8]) {
        if self.buffer_len > 0 {
            let take = core::cmp::min(16 - self.buffer_len, data.len());
            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&data[..take]);
            self.buffer_len += take;
            data = &data[take..];
            if self.buffer_len == 16 {
                let full = self.buffer;
                self.blocks(&[full], HIBIT);
                self.buffer_len = 0;
            }
        }
        let (full, rest) = data.as_chunks::<16>();
        self.blocks(full, HIBIT);
        if !rest.is_empty() {
            self.buffer[..rest.len()].copy_from_slice(rest);
            self.buffer_len = rest.len();
        }
    }

    pub(crate) fn finalize(&mut self) -> [u8; 16] {
        if self.buffer_len > 0 {
            let mut last = [0u8; 16];
            last[..self.buffer_len].copy_from_slice(&self.buffer[..self.buffer_len]);
            last[self.buffer_len] = 1;
            self.blocks(&[last], 0);
        }
        let [mut h0, mut h1, mut h2] = self.h;
        /*
         * Two carry passes leave h fully reduced below 2^130 with each limb
         * in range; then h - p is computed and kept, by mask, only when it
         * does not borrow, so h ends below p = 2^130 - 5.
         */
        for _ in 0..2 {
            h2 += h1 >> 44;
            h1 &= M44;
            h0 += (h2 >> 42) * 5;
            h2 &= M42;
            h1 += h0 >> 44;
            h0 &= M44;
        }
        let g0 = h0 + 5;
        let g1 = h1 + (g0 >> 44);
        let g2 = (h2 + (g1 >> 44)).wrapping_sub(1 << 42);
        let keep_g = (g2 >> 63).wrapping_sub(1);
        h0 = (h0 & !keep_g) | (g0 & M44 & keep_g);
        h1 = (h1 & !keep_g) | (g1 & M44 & keep_g);
        h2 = (h2 & !keep_g) | (g2 & keep_g);
        let h = (h0 as u128).wrapping_add((h1 as u128) << 44).wrapping_add((h2 as u128) << 88);
        let pad = (self.pad[0] as u128) | ((self.pad[1] as u128) << 64);
        h.wrapping_add(pad).to_le_bytes()
    }
}
