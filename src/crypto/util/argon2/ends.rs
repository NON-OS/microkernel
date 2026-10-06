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

//! The first two blocks of each lane, from H0, and the tag from the last
//! column, RFC 9106 section 3.2 steps 1 to 4 and 7 to 8.

use super::blake2b::Blake2b;
use super::block::Block;
use super::hprime::h_prime;
use super::params::Params;
use crate::crypto::constant_time::secure_zero;

/// What is hashed besides the cost parameters.
pub struct Inputs<'a> {
    pub password: &'a [u8],
    pub salt: &'a [u8],
    pub secret: &'a [u8],
    pub ad: &'a [u8],
    /// 0 for Argon2d, 1 for Argon2i, 2 for Argon2id.
    pub y: u32,
}

const VERSION: u32 = 0x13;

pub(super) fn h0(inputs: &Inputs, params: Params, tag_len: usize) -> [u8; 64] {
    let mut h = Blake2b::new(64);
    for word in [params.p, tag_len as u32, params.m_kib, params.t, VERSION, inputs.y] {
        h.update(&word.to_le_bytes());
    }
    for field in [inputs.password, inputs.salt, inputs.secret, inputs.ad] {
        h.update(&(field.len() as u32).to_le_bytes());
        h.update(field);
    }
    let mut out = [0u8; 64];
    h.finalize(&mut out);
    out
}

/// Block `column` (0 or 1) of `lane`: H'^1024(H0 || column || lane).
pub(super) fn first_block(h0: &[u8; 64], column: u32, lane: u32, out: &mut Block) {
    let mut bytes = [0u8; 1024];
    h_prime(&mut bytes, &[h0, &column.to_le_bytes(), &lane.to_le_bytes()]);
    for (w, chunk) in out.iter_mut().zip(bytes.chunks_exact(8)) {
        *w = u64::from_le_bytes(chunk.try_into().unwrap_or([0; 8]));
    }
    secure_zero(&mut bytes);
}

/// The tag: H'^T over the XOR of every lane's last block.
pub(super) fn tag(last: &Block, out: &mut [u8]) {
    let mut bytes = [0u8; 1024];
    for (chunk, w) in bytes.chunks_exact_mut(8).zip(last.iter()) {
        chunk.copy_from_slice(&w.to_le_bytes());
    }
    h_prime(out, &[&bytes]);
    secure_zero(&mut bytes);
}
