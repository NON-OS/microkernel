// NONOS Operating System (AGPL-3.0-or-later)
//! A sound NTB of about a power of two in bytes, 64 to 4096, NTB16 or
//! NTB32: the header, then one to three NDPs chained in order, each after
//! its datagrams (14 to 200 bytes) and listing them, then its zero entry.
//! Everything starts on a 4-byte boundary.

use super::rng::Rng;
use super::seal::seal;
use super::shape::{Shape, N16, N32};

pub struct Block {
    pub raw: Vec<u8>,
    pub s: &'static Shape,
    /// Each NDP's offset in chain order, and the datagrams it lists.
    pub ndps: Vec<(usize, Vec<(usize, usize)>)>,
    pub rx_max: usize,
}

impl Block {
    pub fn put(&mut self, at: usize, v: u64, width: usize) {
        self.raw[at..at + width].copy_from_slice(&v.to_le_bytes()[..width]);
    }

    /// A length or index field of this format's width.
    pub fn field(&mut self, at: usize, v: u64) {
        let w = if self.s.wide { 4 } else { 2 };
        self.put(at, v, w);
    }
}

pub fn block(r: &mut Rng, many: bool) -> Block {
    let s = if r.next() & 1 == 0 { &N16 } else { &N32 };
    let target = 1usize << r.range(6, 12);
    let mut b = Block { raw: vec![0; 4096 + 512], s, ndps: Vec::new(), rx_max: 0 };
    let mut at = s.nth_len.next_multiple_of(4);
    for _ in 0..r.range(1, if many { 2 } else { 3 }) {
        // A new NDP only when one more datagram and its entries still fit.
        if !b.ndps.is_empty() && at + 300 + 16 * 13 > 4096 {
            break;
        }
        let mut grams = Vec::new();
        for _ in 0..if many { r.range(41, 60) } else { r.range(1, 12) } {
            let len = if many { 14 } else { r.range(14, (target / 12).clamp(60, 300)) };
            // Room left for this NDP's entries: one transfer holds the block.
            if !grams.is_empty() && (at + len + 16 * 13 > 4096 || !many && at >= target) {
                break;
            }
            grams.push((at, len));
            at = (at + len).next_multiple_of(4);
        }
        let len = s.ndp_header + (grams.len() + 1) * s.entry;
        b.ndps.push((at, grams));
        at = (at + len).next_multiple_of(4);
        if at >= target && !many {
            break;
        }
    }
    seal(r, &mut b, at);
    b
}
