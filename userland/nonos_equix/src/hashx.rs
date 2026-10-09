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


//! HashX: a seed picks a random program, and the hash of a 64-bit input is
//! that program run over registers SipHash fills from the input.
//!
//! This is the variant Tor and the Anyone fork build: counter input, not
//! block input, and an output of eight bytes.

use crate::blake2b::Blake2b;
use crate::execute::execute;
use crate::program::{generate, Program};
use crate::siphash::{round, siphash24_ctr_state512, SipState};

/// The salt the reference compiles in when none is configured.
const SALT: &[u8; 16] = b"HashX v1\0\0\0\0\0\0\0\0";

pub struct HashX {
    program: Program,
    keys: SipState,
}

impl HashX {
    /// The hash function a seed selects, or `None` for the rare seed whose
    /// program fails the uniformity rules. Equi-X treats that as a challenge
    /// with no solutions, and the solver moves on to the next nonce.
    pub fn new(seed: &[u8]) -> Option<Self> {
        let mut hash = Blake2b::new(64, SALT);
        hash.update(seed);
        let mut bytes = [0u8; 64];
        hash.finish(&mut bytes);
        let word = |i: usize| {
            let mut w = [0u8; 8];
            w.copy_from_slice(&bytes[i * 8..i * 8 + 8]);
            u64::from_le_bytes(w)
        };
        let program_key = SipState { v0: word(0), v1: word(1), v2: word(2), v3: word(3) };
        let keys = SipState { v0: word(4), v1: word(5), v2: word(6), v3: word(7) };
        let program = generate(&program_key)?;
        Some(Self { program, keys })
    }

    /// The hash of one input, as the little-endian reading of the reference's
    /// eight output bytes.
    pub fn hash(&self, input: u64) -> u64 {
        let mut r = siphash24_ctr_state512(&self.keys, input);
        execute(&self.program, &mut r);
        // Finalization removes the bias toward zero that the multiplications
        // leave behind; one SipRound per four registers is enough.
        r[0] = r[0].wrapping_add(self.keys.v0);
        r[1] = r[1].wrapping_add(self.keys.v1);
        r[6] = r[6].wrapping_add(self.keys.v2);
        r[7] = r[7].wrapping_add(self.keys.v3);
        let [a, b, c, d, e, f, g, h] = &mut r;
        round(a, b, c, d);
        round(e, f, g, h);
        r[0] ^ r[4]
    }
}
