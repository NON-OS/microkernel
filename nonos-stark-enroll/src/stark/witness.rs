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

//! A v3 path as the prover's witness, and a root as its four words.

use stark_proofs::attest::Witness;
use stark_proofs::crypto::stark::field::Fp;

use crate::context::DEPTH;

/* A v3 path: magic, depth byte, the siblings, then direction bits from the leaf up. */
pub(super) fn witness_of(path: &[u8]) -> Option<Witness> {
    let sib_end = 9 + DEPTH * 32;
    if path.len() != sib_end + DEPTH.div_ceil(8) || *path.get(8)? as usize != DEPTH {
        return None;
    }
    let siblings = (0..DEPTH)
        .map(|k| words(path.get(9 + k * 32..9 + k * 32 + 32)?).map(|w| w.map(Fp::from_u64)))
        .collect::<Option<Vec<_>>>()?;
    let dirs = path.get(sib_end..)?;
    let right = (0..DEPTH).map(|k| (dirs[k / 8] >> (k % 8)) & 1 == 1).collect();
    Some(Witness { siblings, right })
}

/// Four little-endian words, refused unless each is canonical.
pub(super) fn words(b: &[u8]) -> Option<[u64; 4]> {
    let mut out = [0u64; 4];
    for (lane, chunk) in out.iter_mut().zip(b.chunks_exact(8)) {
        let mut w = [0u8; 8];
        w.copy_from_slice(chunk);
        *lane = u64::from_le_bytes(w);
        if *lane >= stark_proofs::crypto::stark::field::P {
            return None;
        }
    }
    (b.len() == 32).then_some(out)
}
