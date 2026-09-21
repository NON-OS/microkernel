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

//! An ephemeral X25519 keypair, one per hop, wiped when the hop ends.

use nonos_libc::{crypto_random, crypto_x25519_public, crypto_x25519_shared};

use super::CryptoError;

pub const SCALAR_BYTES: usize = 32;
pub const POINT_BYTES: usize = 32;

/// An ephemeral keypair, one per circuit hop, discarded once the hop is up.
pub struct Ephemeral {
    secret: [u8; SCALAR_BYTES],
    pub public: [u8; POINT_BYTES],
}

impl Ephemeral {
    pub fn generate() -> Result<Self, CryptoError> {
        let mut secret = [0u8; SCALAR_BYTES];
        if crypto_random(secret.as_mut_ptr(), secret.len()) != secret.len() as i64 {
            return Err(CryptoError::Random);
        }
        let mut public = [0u8; POINT_BYTES];
        if crypto_x25519_public(secret.as_ptr(), public.as_mut_ptr()) != POINT_BYTES as i64 {
            return Err(CryptoError::Ecdh);
        }
        Ok(Self { secret, public })
    }

    /// The shared point with `peer`.
    ///
    pub fn shared(&self, peer: &[u8; POINT_BYTES]) -> Result<[u8; POINT_BYTES], CryptoError> {
        let mut out = [0u8; POINT_BYTES];
        let n = crypto_x25519_shared(self.secret.as_ptr(), peer.as_ptr(), out.as_mut_ptr());
        if n != POINT_BYTES as i64 {
            return Err(CryptoError::Ecdh);
        }
        if out.iter().all(|b| *b == 0) {
            return Err(CryptoError::DegenerateKey);
        }
        Ok(out)
    }
}

impl Drop for Ephemeral {
    fn drop(&mut self) {
        for byte in self.secret.iter_mut() {
            /*
             * SAFETY: eK@nonos.systems. The scalar must be gone when the hop
             * is, and a plain store into a value being dropped is removable.
             */
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
