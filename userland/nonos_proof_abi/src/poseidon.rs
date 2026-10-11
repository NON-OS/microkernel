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

//! The chains the isolation statement walks, computed outside the circuit:
//! a domain word in lane 0, then one compression per four words. The circuit
//! constrains exactly these, so the kernel, the prover and every verifier
//! share this one function.

use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::{Fp, P};

/// 32 rounds, the attestation tree's and the device statement's hasher.
fn hasher() -> Poseidon {
    Poseidon::new(5, [Fp::ZERO; RATE])
}

/// `None` when `words` is not whole blocks of four or a word is not below p:
/// a non-canonical word would commit to the same value as its reduction.
pub fn chain(domain: u64, words: &[u64]) -> Option<[u64; 4]> {
    if domain >= P || !words.len().is_multiple_of(RATE) || words.iter().any(|&w| w >= P) {
        return None;
    }
    let h = hasher();
    let mut acc = [Fp::from_u64(domain), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    for block in words.chunks_exact(RATE) {
        let b: [Fp; RATE] = core::array::from_fn(|i| Fp::from_u64(block[i]));
        acc = h.compress(&acc, &b);
    }
    Some(acc.map(Fp::to_u64))
}
