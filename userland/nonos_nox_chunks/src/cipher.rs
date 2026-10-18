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

//! The per-order cipher: keystream word j of chunk i is
//! keccak256(abi.encode(bytes32 K, uint64 i, uint32 j)), j = 0..31, and a
//! cipher chunk is the plain chunk XOR those 32 words. keyCommit =
//! keccak256(abi.encode(K)), which deliver() checks the revealed K against.

use nonos_marketplace_abi::Keccak;

use crate::chunk::CHUNK;

pub fn keystream_word(k: &impl Keccak, key: &[u8; 32], i: u64, j: u32) -> [u8; 32] {
    let mut pre = [0u8; 96];
    pre[..32].copy_from_slice(key);
    pre[56..64].copy_from_slice(&i.to_be_bytes());
    pre[92..].copy_from_slice(&j.to_be_bytes());
    k.keccak256(&pre)
}

/// Chunk `i` under `key`; the same call decrypts.
pub fn xor_chunk(k: &impl Keccak, key: &[u8; 32], i: u64, chunk: &[u8; CHUNK]) -> [u8; CHUNK] {
    let mut out = *chunk;
    // Counted as the u32 it is written as: no word index is ever converted.
    for (j, part) in (0u32..).zip(out.chunks_mut(32)) {
        let ks = keystream_word(k, key, i, j);
        part.iter_mut().zip(ks).for_each(|(b, s)| *b ^= s);
    }
    out
}

pub fn key_commit(k: &impl Keccak, key: &[u8; 32]) -> [u8; 32] {
    k.keccak256(key)
}
