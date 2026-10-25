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

//! The secret key file, as `keys` lays it out.

use nonos_ed25519::KeyPair;
use nonos_mldsa65::{SecretKey, PUBLIC_LEN, SECRET_LEN};

use super::{AuthorityKey, SECRET_FILE_LEN};
use crate::Error;

const MAGIC: &[u8; 8] = b"NSHA\x01\x00\x00\x00";

impl AuthorityKey {
    pub fn to_file(&self) -> Vec<u8> {
        let parts = [&MAGIC[..], &self.ed.private, &self.ml_public[..]];
        [&parts.concat()[..], &self.ml_secret.as_bytes()[..]].concat()
    }

    pub fn from_file(b: &[u8]) -> Result<Self, Error> {
        if b.len() != SECRET_FILE_LEN || &b[..8] != MAGIC {
            return Err(Error::KeyFile);
        }
        let mut seed: [u8; 32] = b[8..40].try_into().map_err(|_| Error::KeyFile)?;
        let ml_public: [u8; PUBLIC_LEN] =
            b[40..40 + PUBLIC_LEN].try_into().map_err(|_| Error::KeyFile)?;
        let sk: &[u8; SECRET_LEN] = b[40 + PUBLIC_LEN..].try_into().map_err(|_| Error::KeyFile)?;
        let ed = KeyPair::from_seed(seed);
        seed.fill(0);
        let ml_secret = SecretKey::from_bytes(sk);
        Ok(AuthorityKey { ed, ml_public: Box::new(ml_public), ml_secret })
    }
}
