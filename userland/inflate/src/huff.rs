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

//! Table-driven Huffman decoding. An entry holds the code length in bits
//! 0..8 (0: no code), extra bits in 8..12, the kind in 12..16 and the
//! value in 16..32. Codes longer than the primary width (log2 of `N`) go
//! through a second-level table in `sub`, whose primary entry has kind
//! `SUB`, the table's offset as value and its width as length.

use alloc::vec::Vec;

pub const LIT: u32 = 0x0000;
pub const LEN: u32 = 0x1000;
pub const EOB: u32 = 0x2000;
pub const SUB: u32 = 0x3000;
pub const BAD: u32 = 0x4000;
pub const KIND: u32 = 0xF000;

pub struct Table<const N: usize> {
    pub primary: [u32; N],
    pub sub: Vec<u32>,
}

impl<const N: usize> Table<N> {
    pub const BITS: u32 = N.trailing_zeros();

    pub fn new() -> Self {
        Table { primary: [0; N], sub: Vec::new() }
    }

    /// The entry for the code at the bottom of `buf`.
    #[inline(always)]
    pub fn lookup(&self, buf: u64) -> u32 {
        let e = self.primary[buf as usize & (N - 1)];
        if e & KIND != SUB {
            return e;
        }
        let at = (buf >> Self::BITS) as usize & ((1usize << (e & 0xFF)) - 1);
        self.sub[(e >> 16) as usize + at]
    }
}
