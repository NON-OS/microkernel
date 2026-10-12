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
use crate::dispute::Dispute;
use crate::terms::OrderTerms;

/// dispute(T, i, c_i, plainHash, cipherProof, leafProof).
pub fn dispute_call(t: &OrderTerms, d: &Dispute) -> Vec<u8> {
    let heads: usize = 14 + 5;
    let at_bytes = 32 * heads;
    // A chunk is CHUNK bytes and a proof a tree's depth: none comes near a saturation.
    let bytes_len = d.cipher_chunk.len().div_ceil(32).saturating_mul(32).saturating_add(32);
    let at_cipher = at_bytes.saturating_add(bytes_len);
    let at_leaf =
        at_cipher.saturating_add(d.cipher_proof.len().saturating_add(1).saturating_mul(32));
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
    out.resize(at_cipher.saturating_add(4), 0);
    for proof in [&d.cipher_proof, &d.leaf_proof] {
        out.extend_from_slice(&num(proof.len() as u128));
        proof.iter().for_each(|w| out.extend_from_slice(w));
    }
    out
}
