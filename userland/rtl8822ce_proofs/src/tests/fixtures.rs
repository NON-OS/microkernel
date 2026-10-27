// NONOS Operating System (AGPL-3.0-or-later)
//! A board as the efuse would describe it: two paths, a crystal of 0x25,
//! factory power indexes and no trims burned; and a packed efuse to fill.

use crate::efuse::facts::{PathPower, Trim};
use crate::efuse::EfuseInfo;
use crate::model::Chip;

pub fn board(cut: u8, rfe: u8) -> EfuseInfo {
    let power = PathPower { cck: [0x3C; 6], bw40: [0x40; 5], diff: 0x02 };
    let tx_5g = [[0xFF; 5]; 2];
    let trim = Trim {
        tx_2g: [0xFF; 3],
        tx_5g,
        thermal: [0xFF; 2],
        pa_bias_2g: [0xFF; 2],
        pa_bias_5g: [0; 2],
    };
    EfuseInfo { cut, paths: 2, rfe, xtal: 0x25, power: [power; 2], trim }
}

/// Where in `c`'s log the write of `v` to `off` is, the first one.
pub fn wrote(c: &Chip, off: usize, v: u32) -> Option<usize> {
    c.writes.borrow().iter().position(|w| w.0 == off && w.2 == v)
}

/// Where in `c`'s log the last write to `off` is.
pub fn last(c: &Chip, off: usize) -> Option<usize> {
    c.writes.borrow().iter().rposition(|w| w.0 == off)
}

/// An efuse never written.
pub fn blank() -> [u8; 512] {
    [0xFF; 512]
}

/// Pack block `blk`'s words (those `word_en` keeps) at `at`; the next free byte.
pub fn put(p: &mut [u8; 512], at: usize, blk: u8, word_en: u8, words: &[[u8; 2]]) -> usize {
    let mut i = at;
    if blk < 16 {
        p[i] = blk << 4 | word_en;
        i += 1;
    } else {
        (p[i], p[i + 1]) = ((blk & 7) << 5 | 0x0F, (blk >> 3) << 4 | word_en);
        i += 2;
    }
    for w in words {
        (p[i], p[i + 1]) = (w[0], w[1]);
        i += 2;
    }
    i
}
