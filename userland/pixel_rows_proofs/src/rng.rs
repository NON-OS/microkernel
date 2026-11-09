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

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed | 1)
    }

    pub fn draw(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// Pixels in every alpha class: clear, opaque, and between, with
    /// premultiplied and out-of-range colour both present.
    pub fn pixels(&mut self, n: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(n * 4);
        for _ in 0..n {
            let r = self.draw();
            let a = match r % 4 {
                0 => 0,
                1 => 0xFF,
                _ => (r >> 8) as u8,
            };
            let rgb = (r >> 16) as u32 & 0x00FF_FFFF;
            v.extend_from_slice(&(u32::from(a) << 24 | rgb).to_le_bytes());
        }
        v
    }
}
