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

//! The preprocessed-periodic root: the keccak wide-periodic Merkle root over the
//! coset low-degree extension of an AIR's periodic columns. It is the constant a
//! preprocessed-periodic verifier bakes, and the value a per-program verifier key
//! binds to the program that produced the wiring. The preprocessed prover
//! (`prove_ext_pre`) commits the periodic tree through the same function here, so a
//! root computed for registration and a root committed inside a proof are the same
//! object by construction, not by agreement.

use alloc::vec::Vec;

use super::super::field::Fp;
use super::super::merkle::{hash_leaf_wide_periodic, MerkleTree};
use super::composition::domain_params_blown;
use super::prove_ext::{extend, periodic_coeffs, Domain};
use super::spec::AirExt;

/// The periodic coefficients and the wide-periodic tree over their coset
/// extension. Both the preprocessed prover and the root helper go through this,
/// so the periodic domain size, the coset, and the leaf and node rules cannot
/// drift between them. The extension itself is never held: leaves are hashed
/// one coset at a time, and the tree above the digests is the same build the
/// materialized committer used, so the root cannot tell which one ran.
pub(super) fn periodic_tree<A: AirExt>(
    air: &A,
    extra_blowup_bits: u32,
) -> (Vec<Vec<Fp>>, MerkleTree) {
    let d = Domain::of(air, extra_blowup_bits);
    let cols = air.periodic_columns();
    let coeffs = periodic_coeffs(&cols, &d);
    // A columnless AIR commits the empty tree, as the materialized committer
    // always did: it took its leaf count from the columns, this path takes it
    // from the domain, and only the empty case can tell them apart.
    if cols.is_empty() {
        return (coeffs, MerkleTree::commit_wide_periodic(&[]));
    }
    let mut digests = alloc::vec![[0u8; 32]; d.n];
    for c in 0..d.blowup {
        let per = extend(&coeffs, &d, c);
        let hashed: Vec<[u8; 32]> = crate::par::map_index(d.t, |i| {
            let row: Vec<Fp> = per.iter().map(|col| col[i]).collect();
            hash_leaf_wide_periodic(&row)
        });
        for (i, h) in hashed.into_iter().enumerate() {
            digests[c + d.blowup * i] = h;
        }
    }
    let pad = alloc::vec![Fp::ZERO; cols.len()];
    let tree = MerkleTree::from_leaf_digests(digests, hash_leaf_wide_periodic(&pad));
    (coeffs, tree)
}

/// The 32-byte preprocessed-periodic root for `air` at the given FRI rate. This is
/// the value a per-program verifier key binds; the preprocessed prover commits the
/// identical tree, so the two never diverge.
pub fn periodic_root<A: AirExt>(air: &A, extra_blowup_bits: u32) -> [u8; 32] {
    periodic_tree(air, extra_blowup_bits).1.root()
}

/// The log2 of the periodic evaluation domain for `air` at this FRI rate: the
/// size each periodic column is extended to. Deterministic public structure,
/// exposed so an out-of-crate committer (for example a parallel one) sizes the
/// domain identically to the serial path and cannot drift from it.
pub fn periodic_domain_log<A: AirExt>(air: &A, extra_blowup_bits: u32) -> u32 {
    domain_params_blown(air, extra_blowup_bits).0
}

#[cfg(test)]
mod tests {
    use super::super::spec::{Air, AirExt};
    use super::*;
    use crate::field::Fp2;

    /// The zero-column AIR: the one shape that can tell the streamed committer
    /// from the materialized one, because only there do "leaf count from the
    /// domain" and "leaf count from the columns" disagree.
    struct NoPeriodic;

    impl Air for NoPeriodic {
        fn log_trace_len(&self) -> u32 {
            4
        }
        fn trace_width(&self) -> usize {
            1
        }
        fn window_size(&self) -> usize {
            2
        }
        fn constraint_degree(&self) -> usize {
            3
        }
        fn num_transition(&self) -> usize {
            1
        }
        fn periodic_columns(&self) -> Vec<Vec<Fp>> {
            Vec::new()
        }
        fn transition(&self, w: &[Fp], _p: &[Fp]) -> Vec<Fp> {
            alloc::vec![w[1] - w[0]]
        }
        fn boundary(&self) -> Vec<(usize, usize, Fp)> {
            Vec::new()
        }
    }

    impl AirExt for NoPeriodic {
        fn transition_ext(&self, w: &[Fp2], _p: &[Fp2]) -> Vec<Fp2> {
            alloc::vec![w[1] - w[0]]
        }
    }

    #[test]
    fn a_columnless_air_commits_the_empty_tree() {
        let (coeffs, tree) = periodic_tree(&NoPeriodic, 1);
        assert!(coeffs.is_empty());
        let empty = MerkleTree::commit_wide_periodic(&[]);
        assert_eq!(
            tree.root(),
            empty.root(),
            "streamed and materialized roots diverge at zero columns"
        );
    }
}
