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
use crate::params::{Digest, RATE, WIDTH};
use crate::poseidon::Poseidon;

/// "NONOSB3H": keeps a hybrid measurement apart from any other use of the state.
const HYBRID_DOMAIN: u64 = 0x4E4F_4E4F_5342_3348;

/*
 * A 32-byte BLAKE3 digest into the tree's field: the domain in lane 0, the four
 * little-endian words in lanes 1 to 4, one permutation, the first four lanes. The
 * same measurement as the STARK tree's, so an image measures to the same value
 * under both. Each word is reduced, which is exact: 2^64 - p is below p.
 */
pub fn measure_digest_hybrid(h: &Poseidon, digest: &[u8; 32]) -> Digest {
    let mut state = [Fp::ZERO; WIDTH];
    state[0] = Fp::from_u64(HYBRID_DOMAIN);
    for (lane, word) in state[1..=RATE].iter_mut().zip(digest.chunks_exact(8)) {
        let mut w = [0u8; 8];
        w.copy_from_slice(word);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    let out = h.permute(state);
    let mut d = [Fp::ZERO; RATE];
    d.copy_from_slice(&out[..RATE]);
    d
}
