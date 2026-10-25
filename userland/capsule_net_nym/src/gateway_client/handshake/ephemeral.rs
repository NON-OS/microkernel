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

use super::sizes::{EPHEMERAL_BYTES, SALT_BYTES};
use super::wire::HandshakeError;
use crate::crypto::ecdh::x25519_public;
use crate::crypto::random::fill_random;

/// A throwaway key pair and salt. Fresh every exchange: reusing either would
/// let two sessions derive the same gateway key.
pub struct Ephemeral {
    pub secret: [u8; 32],
    pub public: [u8; EPHEMERAL_BYTES],
    pub salt: [u8; SALT_BYTES],
}

/// The secret is wiped when the exchange that drew it is over, whichever way
/// it ended.
impl Drop for Ephemeral {
    fn drop(&mut self) {
        crate::crypto::wipe::wipe(&mut self.secret);
    }
}

pub fn new_ephemeral() -> Result<Ephemeral, HandshakeError> {
    let mut eph =
        Ephemeral { secret: [0u8; 32], public: [0u8; EPHEMERAL_BYTES], salt: [0u8; SALT_BYTES] };
    fill_random(&mut eph.secret).map_err(|_| HandshakeError::Crypto)?;
    x25519_public(&eph.secret, &mut eph.public).map_err(|_| HandshakeError::Crypto)?;
    fill_random(&mut eph.salt).map_err(|_| HandshakeError::Crypto)?;
    Ok(eph)
}
