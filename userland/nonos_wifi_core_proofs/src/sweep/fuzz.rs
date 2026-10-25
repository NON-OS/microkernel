// NONOS Operating System (AGPL-3.0-or-later)

//! A seeded xorshift generator and the mutations the fuzz proofs apply to a
//! well-formed frame, so every run walks the same inputs and a failure names
//! a round that reproduces.

pub struct Rng(u64);

impl Rng {
    pub fn seeded(seed: u64) -> Self {
        Self(seed | 1)
    }

    pub fn word(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    pub fn byte(&mut self) -> u8 {
        (self.word() >> 24) as u8
    }

    /// A value in 0..n; n must not be zero.
    pub fn below(&mut self, n: usize) -> usize {
        (self.word() % n as u64) as usize
    }

    /// `seed` with one to four edits: a bit flipped, a byte set to an edge
    /// value, the frame cut short, bytes inserted or appended, or a run of
    /// bytes copied over another. Lengths and counts are the usual target.
    pub fn mutate(&mut self, seed: &[u8]) -> Vec<u8> {
        let mut f = seed.to_vec();
        for _ in 0..1 + self.below(4) {
            let at = self.below(f.len().max(1));
            match self.below(6) {
                0 if !f.is_empty() => f[at] ^= 1 << self.below(8),
                1 if !f.is_empty() => f[at] = [0, 1, 0x7F, 0x80, 0xFF][self.below(5)],
                2 => f.truncate(at),
                3 => f.insert(at.min(f.len()), self.byte()),
                4 => f.extend((0..self.below(40)).map(|_| self.byte())),
                _ if f.len() > 2 => {
                    let (from, n) = (self.below(f.len()), 1 + self.below(8));
                    let run: Vec<u8> = f.iter().skip(from).take(n).copied().collect();
                    let to = self.below(f.len());
                    for (i, b) in run.into_iter().enumerate() {
                        if let Some(d) = f.get_mut(to + i) {
                            *d = b;
                        }
                    }
                }
                _ => {}
            }
        }
        f
    }

    /// Bytes of a random length below `max`, every one random.
    pub fn noise(&mut self, max: usize) -> Vec<u8> {
        (0..self.below(max)).map(|_| self.byte()).collect()
    }
}

/// Whether `m` changed a byte `seed` already had, or cut it short: an edit a
/// MIC over the original must catch. Bytes only appended past its end are not.
pub fn altered(seed: &[u8], m: &[u8]) -> bool {
    m.len() < seed.len() || m[..seed.len()] != *seed
}
