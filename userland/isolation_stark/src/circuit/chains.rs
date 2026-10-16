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

//! The two chains: H's from "NONOSISO", A's from "NONOSPOL", each a path
//! whose every sibling is on the right, each leaf pinned to its domain.

use alloc::vec::Vec;

use nonos_proof_abi::{BUCKETS, HIST_WORDS, ISO_DOMAIN, POL_DOMAIN};
use nonos_stark::air::{MultiMembership, Opening, RATE};
use nonos_stark::field::Fp;

use super::layout::{A_BLOCKS, H_BLOCKS};
use crate::params::{hasher, STATEMENT_LOG_ROUNDS};
use crate::statement::Witness;

fn blocks(words: &[u64], n: usize) -> Vec<[Fp; RATE]> {
    (0..n)
        .map(|m| core::array::from_fn(|i| Fp::from_u64(words.get(4 * m + i).copied().unwrap_or(0))))
        .collect()
}

fn chain(domain: u64, sibs: Vec<[Fp; RATE]>) -> MultiMembership {
    let leaf = [Fp::from_u64(domain), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let h = hasher();
    let root = sibs.iter().fold(leaf, |acc, s| h.compress(&acc, s));
    let directions = alloc::vec![false; sibs.len()];
    let opening = Opening { leaf, root, siblings: sibs, directions };
    // The leaf is a witness cell like any other: pinned, or a prover moves it.
    let pins = (0..RATE).map(|lane| (lane, 0, leaf[lane])).collect();
    MultiMembership::new_witness_bound(hasher(), STATEMENT_LOG_ROUNDS, alloc::vec![opening], pins)
        .with_split()
}

/// H's chain over the histogram, two zeros and the salt; zeros for a shape.
pub(super) fn h_chain(w: Option<&Witness>) -> MultiMembership {
    let mut words: Vec<u64> = alloc::vec![0; 4 * H_BLOCKS];
    if let Some(w) = w {
        words[..HIST_WORDS].iter_mut().zip(&w.hist).for_each(|(o, &c)| *o = u64::from(c));
        words[4 * H_BLOCKS - 4..].copy_from_slice(&w.salt);
    }
    chain(ISO_DOMAIN, blocks(&words, H_BLOCKS))
}

/// A's chain over the set's 513 bits and three zeros.
pub(super) fn a_chain(w: Option<&Witness>) -> MultiMembership {
    let bits: Vec<u64> = (0..BUCKETS)
        .map(|b| w.map_or(0, |w| u64::from(nonos_proof_abi::allows(&w.set, b))))
        .collect();
    chain(POL_DOMAIN, blocks(&bits, A_BLOCKS))
}
