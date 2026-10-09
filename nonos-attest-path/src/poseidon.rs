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

use crate::field::Fp;
use crate::params::{Digest, RATE, RC_DOMAIN, ROUNDS, WIDTH};

/*
 * The width-8 Poseidon the policy tree is built with: x^7 on every lane every
 * round, a Cauchy MDS matrix, round constants from BLAKE3. It must be the same
 * function as the STARK tree's, bit for bit, or every root differs; the known
 * answers in tests/kat_tests.rs hold it to the digest the deployed
 * PoseidonGoldilocks.commitNote is pinned to.
 */
pub struct Poseidon {
    mds: [[Fp; WIDTH]; WIDTH],
    rc: [[Fp; WIDTH]; ROUNDS],
}

impl Poseidon {
    pub fn new() -> Poseidon {
        let mut mds = [[Fp::ZERO; WIDTH]; WIDTH];
        for (i, row) in mds.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let x = Fp::from_u64(i as u64);
                let y = Fp::from_u64((WIDTH + j) as u64);
                *cell = x.sub(y).inv();
            }
        }
        let mut rc = [[Fp::ZERO; WIDTH]; ROUNDS];
        for (r, row) in rc.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let mut h = blake3::Hasher::new();
                h.update(RC_DOMAIN);
                h.update(&(r as u64).to_le_bytes());
                h.update(&(j as u64).to_le_bytes());
                let b = h.finalize();
                let mut w = [0u8; 8];
                w.copy_from_slice(&b.as_bytes()[..8]);
                *cell = Fp::from_u64(u64::from_le_bytes(w));
            }
        }
        Poseidon { mds, rc }
    }

    pub(crate) fn round(&self, state: &[Fp; WIDTH], r: usize) -> [Fp; WIDTH] {
        let mut sbox = [Fp::ZERO; WIDTH];
        for (s, v) in sbox.iter_mut().zip(state.iter()) {
            *s = v.pow(7);
        }
        let mut out = [Fp::ZERO; WIDTH];
        let rc = self.rc.get(r).copied().unwrap_or([Fp::ZERO; WIDTH]);
        for ((o, row), c) in out.iter_mut().zip(self.mds.iter()).zip(rc.iter()) {
            let mut acc = *c;
            for (m, s) in row.iter().zip(sbox.iter()) {
                acc = acc.add(m.mul(*s));
            }
            *o = acc;
        }
        out
    }

    pub fn permute(&self, mut state: [Fp; WIDTH]) -> [Fp; WIDTH] {
        for r in 0..ROUNDS {
            state = self.round(&state, r);
        }
        state
    }

    /// The tree's node hash: left then right in the state, permute, first half.
    pub fn compress(&self, left: &Digest, right: &Digest) -> Digest {
        let mut state = [Fp::ZERO; WIDTH];
        state[..RATE].copy_from_slice(left);
        state[RATE..].copy_from_slice(right);
        let out = self.permute(state);
        let mut d = [Fp::ZERO; RATE];
        d.copy_from_slice(&out[..RATE]);
        d
    }

    #[cfg(test)]
    pub(crate) fn rc(&self, r: usize, j: usize) -> Fp {
        self.rc[r][j]
    }

    #[cfg(test)]
    pub(crate) fn mds(&self, i: usize, j: usize) -> Fp {
        self.mds[i][j]
    }
}

impl Default for Poseidon {
    fn default() -> Self {
        Poseidon::new()
    }
}
