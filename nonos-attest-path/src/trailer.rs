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
use crate::params::{Digest, MAX_DEPTH, RATE};

/// The path trailer. A STARK trailer (`NZKSTRK1`, `NZKSTRK2`) has another magic
/// and is refused whole, never read as a path.
pub const MAGIC: [u8; 8] = *b"NZKPATH1";

/*
 * Layout: MAGIC, one depth byte, `depth` siblings of four little-endian words,
 * then the direction bits packed from the leaf up, bit k of byte k/8. Nothing
 * may follow. Every word must be canonical and the unused bits of the last
 * direction byte must be zero, so each path has exactly one encoding.
 */
pub(crate) struct Path<'a> {
    pub depth: usize,
    siblings: &'a [u8],
    dirs: &'a [u8],
}

pub(crate) fn parse(t: &[u8], depth: usize) -> Option<Path<'_>> {
    if depth == 0 || depth > MAX_DEPTH {
        return None;
    }
    let dir_bytes = depth.div_ceil(8);
    let sib_end = 9 + depth * 32;
    if t.len() != sib_end + dir_bytes || t.get(..8)? != MAGIC || *t.get(8)? as usize != depth {
        return None;
    }
    let dirs = t.get(sib_end..)?;
    let spare = dir_bytes * 8 - depth;
    if spare != 0 && (dirs.last()? >> (8 - spare)) != 0 {
        return None;
    }
    Some(Path { depth, siblings: t.get(9..sib_end)?, dirs })
}

impl Path<'_> {
    pub fn sibling(&self, k: usize) -> Option<Digest> {
        bytes_to_digest(self.siblings.get(k * 32..k * 32 + 32)?)
    }

    /// True when the node at level `k` is the right child.
    pub fn right(&self, k: usize) -> Option<bool> {
        Some((self.dirs.get(k / 8)? >> (k % 8)) & 1 == 1)
    }
}

pub(crate) fn bytes_to_digest(b: &[u8]) -> Option<Digest> {
    if b.len() != 32 {
        return None;
    }
    let mut d = [Fp::ZERO; RATE];
    for (lane, word) in d.iter_mut().zip(b.chunks_exact(8)) {
        let mut w = [0u8; 8];
        w.copy_from_slice(word);
        *lane = Fp::canonical(u64::from_le_bytes(w))?;
    }
    Some(d)
}

/// A digest as the trailer and the compiled-in root carry it.
pub fn digest_to_bytes(d: &Digest) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (chunk, lane) in out.chunks_exact_mut(8).zip(d.iter()) {
        chunk.copy_from_slice(&lane.value().to_le_bytes());
    }
    out
}
