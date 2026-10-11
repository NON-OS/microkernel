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
    let heads = 14 + 5;
    let at_bytes = 32 * heads;
    let bytes_len = 32 + d.cipher_chunk.len().div_ceil(32) * 32;
    let at_cipher = at_bytes + bytes_len;
    let at_leaf = at_cipher + 32 * (1 + d.cipher_proof.len());
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
    out.resize(4 + at_cipher, 0);
    for proof in [&d.cipher_proof, &d.leaf_proof] {
        out.extend_from_slice(&num(proof.len() as u128));
        proof.iter().for_each(|w| out.extend_from_slice(w));
    }
    out
}
