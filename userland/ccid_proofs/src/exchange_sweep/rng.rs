// NONOS Operating System (AGPL-3.0-or-later)
//! The seeded generator the CCID exchange sweep draws from.

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    pub fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + (self.next() % (hi - lo + 1) as u64) as usize
    }

    pub fn one_of(&mut self, v: &[usize]) -> usize {
        v[self.range(0, v.len() - 1)]
    }
}

/// Fewer rounds under Miri; the full count is the native sweep's.
pub fn rounds(full: u64) -> u64 {
    if cfg!(miri) {
        200
    } else {
        full
    }
}
