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

//! dispute()'s calldata: T inline, then i, the offset of c_i, plainHash and
//! the offsets of the two proofs in the head; then c_i as bytes and each
//! proof as bytes32[] in the tail.

use alloc::vec::Vec;

use crate::calls::{head, num, DISPUTE};
use crate::chunk::CHUNK;
use crate::dispute::Dispute;
use crate::terms::OrderTerms;

/// dispute(T, i, c_i, plainHash, cipherProof, leafProof); None for a chunk
/// past CHUNK bytes or an offset that does not fit, never a false offset.
pub fn dispute_call(t: &OrderTerms, d: &Dispute) -> Option<Vec<u8>> {
    if d.cipher_chunk.len() > CHUNK {
        return None;
    }
    let at_bytes: usize = 32 * (14 + 5);
    let bytes_len = d.cipher_chunk.len().div_ceil(32).checked_mul(32)?.checked_add(32)?;
    let at_cipher = at_bytes.checked_add(bytes_len)?;
    let at_leaf = at_cipher.checked_add(d.cipher_proof.len().checked_add(1)?.checked_mul(32)?)?;
    let rest = [
        num(d.index as u128),
        num(at_bytes as u128),
        d.plain_hash,
        num(at_cipher as u128),
        num(at_leaf as u128),
    ];
    let mut out = head(DISPUTE, t, &rest);
    out.extend_from_slice(&num(d.cipher_chunk.len() as u128));
    out.extend_from_slice(&d.cipher_chunk);
    out.resize(at_cipher.checked_add(4)?, 0);
    for proof in [&d.cipher_proof, &d.leaf_proof] {
        out.extend_from_slice(&num(proof.len() as u128));
        proof.iter().for_each(|w| out.extend_from_slice(w));
    }
    Some(out)
}
