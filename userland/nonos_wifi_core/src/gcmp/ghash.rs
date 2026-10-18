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

//! GHASH (NIST SP 800-38D, sections 6.3 and 6.4): blocks multiplied into
//! an accumulator in GF(2^128) under the hash subkey H, with the bit order
//! the standard defines (bit 0 is the most significant bit of the first
//! octet) and R = 11100001 || 0^120. The multiply runs over every bit of
//! its operand whatever its value, so its time does not depend on H.

pub struct Ghash {
    h: u128,
    y: u128,
}

/// Algorithm 1 of SP 800-38D: X * Y in GF(2^128).
fn mul(x: u128, y: u128) -> u128 {
    const R: u128 = 0xe1 << 120;
    let (mut z, mut v) = (0u128, y);
    for i in 0..128 {
        let bit = (x >> (127 - i)) & 1;
        z ^= v & bit.wrapping_neg();
        let lsb = v & 1;
        v = (v >> 1) ^ (R & lsb.wrapping_neg());
    }
    z
}

impl Ghash {
    pub fn new(h: &[u8; 16]) -> Self {
        Self { h: u128::from_be_bytes(*h), y: 0 }
    }

    /// The input zero-padded to whole blocks, as GCM pads A and C.
    pub fn update_padded(&mut self, data: &[u8]) {
        for chunk in data.chunks(16) {
            let mut b = [0u8; 16];
            b[..chunk.len()].copy_from_slice(chunk);
            self.y = mul(self.y ^ u128::from_be_bytes(b), self.h);
        }
    }

    /// The final block: len(A) || len(C), in bits; then the value.
    pub fn finish(mut self, aad_len: usize, ct_len: usize) -> [u8; 16] {
        let lens = ((aad_len as u128 * 8) << 64) | (ct_len as u128 * 8);
        self.y = mul(self.y ^ lens, self.h);
        self.y.to_be_bytes()
    }
}
