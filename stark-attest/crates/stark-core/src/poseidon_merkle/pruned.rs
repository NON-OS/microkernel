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

//! The Poseidon tree with its bottom cut off. A per-column tree over the
//! evaluation domain stores two nodes per leaf, which for a wide trace is
//! tens of gigabytes of digests serving thirty-two openings. Keeping only the
//! levels above a cut divides that by 2^cut; a query recomputes its own
//! subtree from the leaf values at open time. Root and paths are the full
//! tree's, node for node: this changes where digests live, never what they
//! are.

use super::super::air::{Poseidon, RATE};
use super::super::field::Fp;
use alloc::vec::Vec;

pub struct PrunedPoseidonTree {
    /// Levels from the cut upward, `upper[0]` being the level with
    /// `padded_len >> cut` nodes.
    upper: Vec<Vec<[Fp; RATE]>>,
    cut: u32,
    padded_len: usize,
    root: [Fp; RATE],
}

impl PrunedPoseidonTree {
    /// Commit to `leaves`, keeping only the levels at and above `cut`. The
    /// digests below the cut are computed, folded and dropped; the result is
    /// the same commitment `PoseidonMerkleTree::commit` builds.
    pub fn commit(hasher: &Poseidon, leaves: &[[Fp; RATE]], cut: u32) -> PrunedPoseidonTree {
        let mut padded: Vec<[Fp; RATE]> = leaves.to_vec();
        if padded.is_empty() {
            padded.push([Fp::ZERO; RATE]);
        }
        while !padded.len().is_power_of_two() {
            padded.push([Fp::ZERO; RATE]);
        }
        let padded_len = padded.len();
        let cut = cut.min(padded_len.trailing_zeros());

        // Fold each 2^cut chunk to its level-`cut` node independently.
        let chunk = 1usize << cut;
        let mut level: Vec<[Fp; RATE]> = crate::par::map_index(padded_len >> cut, |b| {
            let mut nodes: Vec<[Fp; RATE]> = padded[b * chunk..(b + 1) * chunk].to_vec();
            while nodes.len() > 1 {
                let half = nodes.len() / 2;
                let mut next = Vec::with_capacity(half);
                for i in 0..half {
                    next.push(hasher.compress(&nodes[2 * i], &nodes[2 * i + 1]));
                }
                nodes = next;
            }
            nodes[0]
        });

        let mut upper = Vec::new();
        upper.push(level.clone());
        while level.len() > 1 {
            let half = level.len() / 2;
            let next: Vec<[Fp; RATE]> =
                crate::par::map_index(half, |i| hasher.compress(&level[2 * i], &level[2 * i + 1]));
            level = next;
            upper.push(level.clone());
        }
        let root = upper.last().and_then(|t| t.first()).copied().unwrap_or([Fp::ZERO; RATE]);
        PrunedPoseidonTree { upper, cut, padded_len, root }
    }

    pub fn root(&self) -> [Fp; RATE] {
        self.root
    }

    pub fn len(&self) -> usize {
        self.padded_len
    }

    pub fn is_empty(&self) -> bool {
        self.padded_len == 0
    }

    /// The full authentication path for `index`. The caller supplies the leaf
    /// values of the 2^cut chunk containing it, in order; the levels below the
    /// cut are rebuilt from them, the levels above are read. The path is the
    /// one the unpruned tree would have returned.
    pub fn open_with(
        &self,
        hasher: &Poseidon,
        index: usize,
        chunk_leaves: &[[Fp; RATE]],
    ) -> Vec<[Fp; RATE]> {
        let mut path = Vec::new();
        if index >= self.padded_len {
            return path;
        }
        let mut nodes: Vec<[Fp; RATE]> = chunk_leaves.to_vec();
        let mut idx = index & ((1usize << self.cut) - 1);
        while nodes.len() > 1 {
            path.push(nodes[idx ^ 1]);
            let half = nodes.len() / 2;
            let mut next = Vec::with_capacity(half);
            for i in 0..half {
                next.push(hasher.compress(&nodes[2 * i], &nodes[2 * i + 1]));
            }
            nodes = next;
            idx >>= 1;
        }
        let mut idx = index >> self.cut;
        for level in &self.upper {
            if level.len() <= 1 {
                break;
            }
            path.push(level[idx ^ 1]);
            idx >>= 1;
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::super::tree::PoseidonMerkleTree;
    use super::*;

    /// The pruned tree is the full tree with different storage: same root,
    /// same path at every index, or the pruning changed the commitment.
    #[test]
    fn pruned_matches_full_at_every_index() {
        let h = Poseidon::new(5, [Fp::ZERO; RATE]);
        let leaves: Vec<[Fp; RATE]> = (0..64u64)
            .map(|i| core::array::from_fn(|k| Fp::from_u64(i * 31 + k as u64 + 1)))
            .collect();
        let full = PoseidonMerkleTree::commit(&h, &leaves);
        let pruned = PrunedPoseidonTree::commit(&h, &leaves, 3);
        assert_eq!(full.root(), pruned.root(), "roots diverge");
        for p in 0..64usize {
            let base = p & !7;
            let chunk = &leaves[base..base + 8];
            assert_eq!(full.open(p), pruned.open_with(&h, p, chunk), "path diverges at {p}");
        }
    }
}
