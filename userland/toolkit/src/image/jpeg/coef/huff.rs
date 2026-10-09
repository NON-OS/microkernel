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

use crate::image::jpeg::bits::{extend, BitReader};
use crate::image::jpeg::coef::refuse::CORRUPT_DATA;
use crate::image::jpeg::dht::HuffmanTable;
use crate::image::jpeg::huffman::decode_symbol;
use crate::image::types::DecodeError;

const FAST: u32 = 9;

/// A Huffman table with a lookup for codes up to 9 bits: entry = length
/// << 8 | symbol, 0 where a longer code starts.
pub struct Coder {
    pub table: HuffmanTable,
    lut: [u16; 1 << FAST],
}

impl Coder {
    pub fn new(table: HuffmanTable) -> Coder {
        let mut lut = [0u16; 1 << FAST];
        let (mut code, mut k) = (0u32, 0usize);
        for len in 1..=16u32 {
            for _ in 0..table.bits[len as usize] {
                if code >= 1 << len || k >= table.total {
                    return Coder { table, lut };
                }
                if len <= FAST {
                    let first = (code << (FAST - len)) as usize;
                    for e in &mut lut[first..first + (1 << (FAST - len))] {
                        *e = ((len as u16) << 8) | table.huffval[k] as u16;
                    }
                }
                (code, k) = (code + 1, k + 1);
            }
            code <<= 1;
        }
        Coder { table, lut }
    }

    /// The next symbol: from the lookup when its code is short, else bit
    /// by bit through the canonical table.
    pub fn symbol(&self, br: &mut BitReader) -> Result<u8, DecodeError> {
        if let Ok(p) = br.peek(FAST) {
            let e = self.lut[p as usize];
            if e != 0 {
                br.consume((e >> 8) as u32);
                return Ok(e as u8);
            }
        }
        decode_symbol(br, &self.table)
    }

    /// A symbol giving a bit count, then that many bits as a signed value.
    pub fn value(&self, br: &mut BitReader) -> Result<i32, DecodeError> {
        let t = self.symbol(br)? as u32;
        if t > 15 {
            return Err(CORRUPT_DATA);
        }
        Ok(extend(br.read_bits(t)?, t))
    }
}
