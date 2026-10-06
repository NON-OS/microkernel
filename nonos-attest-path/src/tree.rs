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

use alloc::vec::Vec;

use crate::params::{Digest, MAX_DEPTH};
use crate::poseidon::Poseidon;
use crate::trailer::{digest_to_bytes, MAGIC};

/*
 * The policy tree, for the enroll tool. Level 0 is the leaves; each node above
 * is compress(left, right) of the pair below it, the same node hash `fold` uses,
 * so a trailer this emits is exactly what the gates accept. The caller pads the
 * leaves to 2^depth itself, with the `Pad` kind, so nothing here invents a slot.
 */
pub struct Tree {
    layers: Vec<Vec<Digest>>,
}

impl Tree {
    pub fn commit(h: &Poseidon, leaves: &[Digest], depth: usize) -> Option<Tree> {
        if depth == 0 || depth > MAX_DEPTH || leaves.len() != 1usize << depth {
            return None;
        }
        let mut layers = Vec::with_capacity(depth + 1);
        layers.push(leaves.to_vec());
        while let Some(level) = layers.last().filter(|l| l.len() > 1) {
            let next = level.chunks_exact(2).map(|p| h.compress(&p[0], &p[1])).collect();
            layers.push(next);
        }
        Some(Tree { layers })
    }

    pub fn root(&self) -> Option<Digest> {
        self.layers.last()?.first().copied()
    }

    /// The `NZKPATH1` trailer for slot `index`.
    pub fn trailer(&self, index: usize) -> Option<Vec<u8>> {
        let depth = self.layers.len().checked_sub(1)?;
        if index >= self.layers.first()?.len() {
            return None;
        }
        let mut out = Vec::with_capacity(9 + depth * 32 + depth.div_ceil(8));
        out.extend_from_slice(&MAGIC);
        out.push(u8::try_from(depth).ok()?);
        let mut dirs = alloc::vec![0u8; depth.div_ceil(8)];
        for (k, level) in self.layers.iter().take(depth).enumerate() {
            let at = index >> k;
            out.extend_from_slice(&digest_to_bytes(level.get(at ^ 1)?));
            if at & 1 == 1 {
                *dirs.get_mut(k / 8)? |= 1 << (k % 8);
            }
        }
        out.extend_from_slice(&dirs);
        Some(out)
    }
}
