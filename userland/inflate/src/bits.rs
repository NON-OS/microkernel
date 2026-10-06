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

use super::types::End;

/// LSB-first bit reader over a 64-bit buffer. `cnt` bits of `buf` are
/// valid; the bits above them are zero or already the next input bits, so
/// a refill can OR a whole little-endian word in without masking.
#[derive(Clone, Copy)]
pub struct Bits<'a> {
    pub(super) d: &'a [u8],
    pub(super) pos: usize,
    pub buf: u64,
    pub cnt: u32,
}

impl<'a> Bits<'a> {
    pub fn new(d: &'a [u8]) -> Self {
        Bits { d, pos: 0, buf: 0, cnt: 0 }
    }

    /// Tops the buffer up to at least 56 bits, or to all the input has left.
    #[inline(always)]
    pub fn refill(&mut self) {
        if let Some(w) = self.d.get(self.pos..self.pos.wrapping_add(8)) {
            let w = u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]);
            self.buf |= w << self.cnt;
            self.pos += ((63 - self.cnt) >> 3) as usize;
            self.cnt |= 56;
            return;
        }
        while self.cnt <= 56 {
            let Some(&b) = self.d.get(self.pos) else { return };
            self.buf |= u64::from(b) << self.cnt;
            self.pos += 1;
            self.cnt += 8;
        }
    }

    #[inline(always)]
    pub fn drop_bits(&mut self, n: u32) {
        self.buf >>= n;
        self.cnt -= n;
    }

    /// The next `n` bits (at most 32), or `Truncated` when the input ends.
    #[inline(always)]
    pub fn bits(&mut self, n: u32) -> Result<u32, End> {
        if self.cnt < n {
            self.refill();
            if self.cnt < n {
                return Err(End::Truncated);
            }
        }
        let v = (self.buf & ((1u64 << n) - 1)) as u32;
        self.drop_bits(n);
        Ok(v)
    }
}
