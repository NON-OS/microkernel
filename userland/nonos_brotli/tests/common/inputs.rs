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

/* The inputs the reference encoder compressed into data/streams.dat,
rebuilt here from the same generator: dictionary word text, noise,
and a mix of both with repeats far back in the window. */

const DICT: &[u8] = include_bytes!("../../src/dictionary.dat");
const NDBITS: [u32; 13] = [0, 0, 0, 0, 10, 10, 11, 11, 10, 10, 10, 10, 10];
const SEPS: [&[u8]; 6] = [b" ", b" ", b" ", b", ", b". ", b"\n"];

/// The generator of every test input: a 64-bit LCG, top 31 bits out.
pub struct Lcg(pub u64);

impl Lcg {
    pub fn next(&mut self) -> usize {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) as usize
    }
}

fn text(n: usize, seed: u64) -> Vec<u8> {
    let (mut g, mut out) = (Lcg(seed), Vec::new());
    let mut at = [0usize; 13];
    (4..12).for_each(|l| at[l + 1] = at[l] + (l << NDBITS[l]));
    while out.len() < n {
        let r = g.next();
        let len = 4 + r % 9;
        let start = at[len] + ((r >> 8) % (1 << NDBITS[len])) * len;
        let mut word = DICT[start..start + len].to_vec();
        if (r >> 20) % 7 == 0 && word[0].is_ascii_lowercase() {
            word[0] ^= 32;
        }
        out.extend_from_slice(&word);
        out.extend_from_slice(SEPS[(r >> 24) % 6]);
        if (r >> 28) % 5 == 0 && out.len() > 64 {
            let from = g.next() % (out.len() - 32);
            out.extend_from_within(from..from + 32);
        }
    }
    out.truncate(n);
    out
}

fn noise(n: usize, seed: u64) -> Vec<u8> {
    let mut g = Lcg(seed);
    (0..n).map(|_| g.next() as u8).collect()
}

/// Input `kind` (0 text, 1 noise, 2 mixed) of `n` bytes from `seed`.
pub fn input(kind: u8, n: usize, seed: u64) -> Vec<u8> {
    match kind {
        0 => text(n, seed),
        1 => noise(n, seed),
        _ => {
            let (a, t) = (noise(n / 6, seed), text(n / 3, seed + 1));
            let mut all = [&a[..], &t[..], &a[..], &t[..]].concat();
            all.truncate(n);
            all
        }
    }
}
