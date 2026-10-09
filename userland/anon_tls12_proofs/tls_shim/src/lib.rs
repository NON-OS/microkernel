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


//! nonos_tls's Io trait and RSA verify, on the host.

use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Sign, Pss, RsaPublicKey};
use sha2::{Sha256, Sha384};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SessionError {
    Io,
}

pub trait Io {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError>;
    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError>;
}

/// The crypto pool's RSA verify: scheme 0 is PKCS#1 v1.5 and 1 is PSS with
/// a salt the length of the hash; hash 0 is SHA-256 and 1 SHA-384; `digest`
/// is already hashed.
pub fn verify_rsa(scheme: u8, hashid: u8, spki: &[u8], sig: &[u8], digest: &[u8]) -> bool {
    let Ok(key) = RsaPublicKey::from_public_key_der(spki) else { return false };
    let result = match (scheme, hashid) {
        (0, 0) => key.verify(Pkcs1v15Sign::new::<Sha256>(), digest, sig),
        (0, 1) => key.verify(Pkcs1v15Sign::new::<Sha384>(), digest, sig),
        (1, 0) => key.verify(Pss::new::<Sha256>(), digest, sig),
        (1, 1) => key.verify(Pss::new::<Sha384>(), digest, sig),
        _ => return false,
    };
    result.is_ok()
}
