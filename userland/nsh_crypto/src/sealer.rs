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

//! Sealed frames: `len | ChaCha20-Poly1305(type | body) | tag`, the four
//! length bytes as associated data, and the nonce four zero bytes then the
//! frame's sequence number in this direction. The number is never sent;
//! both sides count. A direction stops after 2^32 frames or 2^32 sealed
//! bytes: version 1 does not rekey, so the session ends instead.

use crate::count::Count;
use crate::{CryptoError, Secret};
use nonos_seal::{seal, TAG_LEN};

pub struct Sealer(Count);
impl Sealer {
    pub fn new(key: Secret<32>) -> Self {
        Sealer::from_count(Count::new(key))
    }

    pub(crate) fn from_count(count: Count) -> Self {
        Sealer(count)
    }

    /// Seal `inner` (the type byte, then the body) behind the length field
    /// `len`, writing ciphertext and tag into `out`.
    pub fn seal(
        &mut self,
        len: [u8; 4],
        inner: &[u8],
        out: &mut [u8],
    ) -> Result<usize, CryptoError> {
        // A frame always has its type byte; the peer refuses one without.
        if inner.is_empty() || u32::from_le_bytes(len) as usize != inner.len() + TAG_LEN {
            return Err(CryptoError::Room);
        }
        let nonce = self.0.nonce(inner.len())?;
        let n =
            seal(self.0.key.expose(), &nonce, &len, inner, out).map_err(|_| CryptoError::Room)?;
        self.0.advance(inner.len());
        Ok(n)
    }
}
