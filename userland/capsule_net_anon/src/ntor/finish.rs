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

//! Finishing a handshake, including the auth check that makes it ntor.

use crate::crypto::{equal, hkdf_sha256, hmac_sha256, CryptoError};

use super::constants::{KEY_MATERIAL_BYTES, M_EXPAND, REPLY_BYTES, T_KEY, T_MAC, T_VERIFY};
use super::create::Handshake;
use super::inputs::{auth_input, secret_input, Parts};

impl Handshake {
    /// Finish against the relay's `Y || auth` reply, returning
    /// `Df || Db || Kf || Kb`.
    pub fn finish(&self, reply: &[u8]) -> Result<[u8; KEY_MATERIAL_BYTES], CryptoError> {
        if reply.len() != REPLY_BYTES {
            return Err(CryptoError::Mac);
        }
        let mut server = [0u8; 32];
        server.copy_from_slice(&reply[..32]);

        let xy = self.ephemeral.shared(&server)?;
        let xb = self.ephemeral.shared(&self.onion_key)?;
        let parts = Parts {
            xy: &xy,
            xb: &xb,
            identity: &self.identity,
            onion_key: &self.onion_key,
            client: &self.ephemeral.public,
            server: &server,
        };

        let secret = secret_input(&parts);
        let mut verify = [0u8; 32];
        hmac_sha256(T_VERIFY, &secret, &mut verify)?;

        let mut auth = [0u8; 32];
        hmac_sha256(T_MAC, &auth_input(&parts, &verify), &mut auth)?;
        if !equal(&auth, &reply[32..]) {
            return Err(CryptoError::Mac);
        }

        let mut keys = [0u8; KEY_MATERIAL_BYTES];
        hkdf_sha256(T_KEY, &secret, M_EXPAND, &mut keys)?;
        Ok(keys)
    }
}
