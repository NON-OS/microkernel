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

//! OpenPGP's arithmetic on this machine: RSA through the crypto service, and
//! Ed25519 in this capsule.

use nonos_openpgp::Verifier;

use super::request::request;

/// The crypto service's PKCS#1 v1.5 scheme.
const PKCS1: u8 = 0;

pub struct Machine;

impl Verifier for Machine {
    fn rsa(&self, n: &[u8], e: &[u8], sig: &[u8], hash: u8, digest: &[u8]) -> bool {
        request(n, e, sig, hash)
            .is_some_and(|r| nonos_tls::verify_rsa(PKCS1, r.hashid, &r.spki, &r.sig, digest))
    }

    fn ed25519(&self, key: &[u8; 32], sig: &[u8; 64], digest: &[u8]) -> bool {
        nonos_ed25519::verify(key, digest, &nonos_ed25519::Signature::from_bytes(sig))
    }
}
