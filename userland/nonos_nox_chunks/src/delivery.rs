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

//! The buyer's check of a delivery (plan/22 step 5): the revealed K must
//! match keyCommit, and every decrypted chunk must hash to its listed plain
//! hash. The first chunk that does not is the one to dispute, within
//! disputeWindow; the content is never handed on unless all pass.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

use crate::chunk::CHUNK;
use crate::cipher::{key_commit, xor_chunk};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Delivered {
    /// The revealed key is not the committed one; nothing was decrypted.
    WrongKey,
    /// Not one whole cipher chunk per hash: not the offer that was checked.
    NotTheOffer,
    /// Chunk `i` decrypts to bytes that are not its listed hash.
    BadChunk(u64),
}

/// The content, `content_len` bytes, once every chunk checks; the
/// ciphertext and hashes are the ones check_offer accepted.
pub fn open_delivery(
    k: &impl Keccak,
    key: &[u8; 32],
    commit: &[u8; 32],
    hashes: &[[u8; 32]],
    ciphertext: &[u8],
    content_len: u64,
) -> Result<Vec<u8>, Delivered> {
    if key_commit(k, key) != *commit {
        return Err(Delivered::WrongKey);
    }
    if hashes.len().checked_mul(CHUNK) != Some(ciphertext.len())
        || content_len > ciphertext.len() as u64
    {
        return Err(Delivered::NotTheOffer);
    }
    let mut out = Vec::with_capacity(ciphertext.len());
    for (i, (c, h)) in ciphertext.chunks(CHUNK).zip(hashes).enumerate() {
        let plain =
            xor_chunk(k, key, i as u64, c.try_into().map_err(|_| Delivered::BadChunk(i as u64))?);
        if k.keccak256(&plain) != *h {
            return Err(Delivered::BadChunk(i as u64));
        }
        out.extend_from_slice(&plain);
    }
    out.truncate(usize::try_from(content_len).unwrap_or(usize::MAX));
    Ok(out)
}
