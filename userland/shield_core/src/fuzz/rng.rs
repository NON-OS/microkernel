// NONOS Operating System (AGPL-3.0-or-later)
//! The rounds' bytes: a xorshift stream from a seed, so a failing round replays exactly.

pub(crate) struct Rng(u64);

impl Rng {
    pub(crate) fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub(crate) fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// A number below `n`, or 0 when `n` is 0.
    pub(crate) fn below(&mut self, n: usize) -> usize {
        let n64 = u64::try_from(n).unwrap_or(u64::MAX);
        match self.next().checked_rem(n64) {
            Some(r) => usize::try_from(r).unwrap_or(0),
            None => 0,
        }
    }

    pub(crate) fn bytes(&mut self, len: usize) -> Vec<u8> {
        (0..len).map(|_| self.next().to_le_bytes()[0]).collect()
    }
}
