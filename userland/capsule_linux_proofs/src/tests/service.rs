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

//! The crypto service's RSA checks, standing in for the IPC call, fed the
//! exact request the capsule builds.

use nonos_openpgp::Verifier;
use rsa::pkcs8::DecodePublicKey;
use rsa::sha2::{Sha256, Sha512};
use rsa::{Pkcs1v15Sign, RsaPublicKey};

use crate::install::pgp::request::request;

/// capsule_crypto's rsa_scheme for scheme 0, standing in for the IPC call.
pub struct Service;

impl Verifier for Service {
    fn rsa(&self, n: &[u8], e: &[u8], sig: &[u8], hash: u8, digest: &[u8]) -> bool {
        let Some(r) = request(n, e, sig, hash) else { return false };
        let Ok(key) = RsaPublicKey::from_public_key_der(&r.spki) else { return false };
        match r.hashid {
            0 => key.verify(Pkcs1v15Sign::new::<Sha256>(), digest, &r.sig).is_ok(),
            2 => key.verify(Pkcs1v15Sign::new::<Sha512>(), digest, &r.sig).is_ok(),
            _ => false,
        }
    }

    fn ed25519(&self, _: &[u8; 32], _: &[u8; 64], _: &[u8]) -> bool {
        false
    }
}
