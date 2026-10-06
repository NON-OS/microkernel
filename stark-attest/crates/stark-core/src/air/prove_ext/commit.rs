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

use super::super::super::field::Fp;
use super::super::super::merkle::hash_leaf_wide;
use super::super::super::merkle::MerkleTree;
use super::coset::extend;
use super::setup::Domain;
use alloc::vec::Vec;

/// The row-wise trace commitment, one coset in memory at a time.
///
/// Leaf `j` hashes every column's value at position `j`, exactly as
/// `commit_wide` does over materialized columns. A coset holds the positions
/// `c + blowup * i`, so its rows land at strides of `blowup` in the digest
/// layer, and after the last coset every leaf has been written once. The tree
/// above the digests is the same `build` either path uses, so the root cannot
/// tell which committer ran.
pub(in crate::air) fn wide_streamed(coeffs: &[Vec<Fp>], d: &Domain) -> MerkleTree {
    let mut digests = alloc::vec![[0u8; 32]; d.n];
    for c in 0..d.blowup {
        let cols = extend(coeffs, d, c);
        let hashed: Vec<[u8; 32]> = crate::par::map_index(d.t, |i| {
            let row: Vec<Fp> = cols.iter().map(|col| col[i]).collect();
            hash_leaf_wide(&row)
        });
        for (i, h) in hashed.into_iter().enumerate() {
            digests[c + d.blowup * i] = h;
        }
    }
    let pad = alloc::vec![Fp::ZERO; d.width];
    MerkleTree::from_leaf_digests(digests, hash_leaf_wide(&pad))
}
