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

//! One direction's key and count: the sequence number that makes each
//! frame's nonce, and the bytes sealed so far.

use crate::{CryptoError, Secret};
use nonos_seal::NONCE_LEN;

pub const MAX_SEALED_FRAMES: u64 = 1 << 32;
pub const MAX_SEALED_BYTES: u64 = 1 << 32;

pub(crate) struct Count {
    pub(crate) key: Secret<32>,
    pub(crate) seq: u64,
    pub(crate) bytes: u64,
}

impl Count {
    pub(crate) fn new(key: Secret<32>) -> Self {
        Count { key, seq: 0, bytes: 0 }
    }

    /// The nonce for the next frame of `n` bytes, or `Limit` once the
    /// direction is spent. Nothing is counted until the frame succeeds.
    pub(crate) fn nonce(&self, n: usize) -> Result<[u8; NONCE_LEN], CryptoError> {
        let bytes = self.bytes.checked_add(n as u64).ok_or(CryptoError::Limit)?;
        if self.seq >= MAX_SEALED_FRAMES || bytes > MAX_SEALED_BYTES {
            return Err(CryptoError::Limit);
        }
        let mut nonce = [0u8; NONCE_LEN];
        nonce[4..].copy_from_slice(&self.seq.to_le_bytes());
        Ok(nonce)
    }

    pub(crate) fn advance(&mut self, n: usize) {
        self.seq += 1;
        self.bytes += n as u64;
    }
}
