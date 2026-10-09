use crate::image::png::deflate::{BitReader, ByteSource};
use crate::image::types::DecodeError;

use super::huffman_fast::{fast_table, FAST_BITS};

pub const MAX_SYMBOLS: usize = 288;

pub struct Huffman {
    counts: [u16; 16],
    symbols: [u16; MAX_SYMBOLS],
    fast: [u16; 1 << FAST_BITS],
}

impl Huffman {
    pub fn from_lengths(lengths: &[u8]) -> Result<Self, DecodeError> {
        let mut counts = [0u16; 16];
        for &l in lengths {
            if l as usize > 15 {
                return Err(DecodeError::Unsupported);
            }
            counts[l as usize] += 1;
        }
        counts[0] = 0;
        let mut offsets = [0u16; 16];
        let mut i = 1usize;
        while i < 15 {
            offsets[i + 1] = offsets[i] + counts[i];
            i += 1;
        }
        let mut symbols = [0u16; MAX_SYMBOLS];
        for (sym, &l) in lengths.iter().enumerate() {
            if l != 0 {
                let o = offsets[l as usize] as usize;
                if o >= MAX_SYMBOLS {
                    return Err(DecodeError::Unsupported);
                }
                symbols[o] = sym as u16;
                offsets[l as usize] += 1;
            }
        }
        Ok(Self { counts, symbols, fast: fast_table(lengths, &counts) })
    }

    /* Short codes come straight from the fast table; longer ones by the
     * canonical walk, a bit at a time from a peeked window, consumed only
     * once a symbol matches. */
    pub fn decode<S: ByteSource>(&self, bits: &mut BitReader<S>) -> Result<u16, DecodeError> {
        let (mut window, avail) = bits.peek16();
        let hit = self.fast[(window & ((1 << FAST_BITS) - 1)) as usize];
        if hit != 0 && (hit & 15) as u32 <= avail {
            bits.consume((hit & 15) as u32);
            return Ok(hit >> 4);
        }
        let (mut code, mut first, mut index, mut len) = (0i32, 0i32, 0i32, 1usize);
        while len < 16 {
            if len as u32 > avail {
                return Err(DecodeError::Truncated);
            }
            code |= (window & 1) as i32;
            window >>= 1;
            let count = self.counts[len] as i32;
            if code - first < count {
                bits.consume(len as u32);
                let pos = (index + (code - first)) as usize;
                return self.symbols.get(pos).copied().ok_or(DecodeError::Unsupported);
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
            len += 1;
        }
        Err(DecodeError::Unsupported)
    }
}
