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

//! The client half of a handshake, and the onion skin it sends.

use crate::crypto::{CryptoError, Ephemeral};

use super::constants::{IDENTITY_BYTES, ONIONSKIN_BYTES};

/// The client's side of an ntor handshake, held between sending the onion
/// skin and receiving the reply.
pub struct Handshake {
    /// SHA-1 of the relay's RSA identity key, as the consensus lists it.
    pub identity: [u8; IDENTITY_BYTES],
    /// The relay's ntor onion key, B, from its microdescriptor.
    pub onion_key: [u8; 32],
    /// The ephemeral x, and X alongside it.
    pub ephemeral: Ephemeral,
}

impl Handshake {
    pub fn new(identity: [u8; IDENTITY_BYTES], onion_key: [u8; 32]) -> Result<Self, CryptoError> {
        Ok(Self { identity, onion_key, ephemeral: Ephemeral::generate()? })
    }

    /// The onion skin to put in a CREATE2 or EXTEND2 cell.
    ///
    pub fn onionskin(&self) -> [u8; ONIONSKIN_BYTES] {
        let mut out = [0u8; ONIONSKIN_BYTES];
        out[..IDENTITY_BYTES].copy_from_slice(&self.identity);
        out[IDENTITY_BYTES..IDENTITY_BYTES + 32].copy_from_slice(&self.onion_key);
        out[IDENTITY_BYTES + 32..].copy_from_slice(&self.ephemeral.public);
        out
    }
}
