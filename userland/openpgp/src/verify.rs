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

//! One detached signature, checked against a pinned keyring.

use super::digest::digest;
use super::key::{Key, Material};
use super::keyring::issuer_key;
use super::mpi::fixed as fill;
use super::refusal::Refusal;
use super::sig::signature;

/// The arithmetic, supplied by the caller. `digest` is what was signed; for
/// RSA it goes inside a PKCS#1 v1.5 DigestInfo for `hash`.
pub trait Verifier {
    fn rsa(&self, n: &[u8], e: &[u8], sig: &[u8], hash: u8, digest: &[u8]) -> bool;
    fn ed25519(&self, key: &[u8; 32], sig: &[u8; 64], digest: &[u8]) -> bool;
}

pub struct Verified {
    pub fingerprint: [u8; 20],
    pub hash: u8,
}

pub fn verify(
    v: &impl Verifier,
    ring: &[Key],
    sig: &[u8],
    data: &[u8],
) -> Result<Verified, Refusal> {
    let s = signature(sig)?;
    if s.kind != 0x00 {
        return Err(Refusal::NotBinary);
    }
    let key = issuer_key(ring, &s.issuer).ok_or(Refusal::UnknownKey)?;
    let d = digest(s.hash, data, s.hashed)?;
    if d.bytes()[..2] != s.left16 {
        return Err(Refusal::QuickCheck);
    }
    let good = match (&key.material, s.algo, s.values.as_slice()) {
        (Material::Rsa { n, e }, 1 | 3, [m]) => v.rsa(n, e, m, s.hash, d.bytes()),
        (Material::Ed25519(pk), 22, [r, sv]) => {
            let mut rs = [0u8; 64];
            fill(&mut rs[..32], r)?;
            fill(&mut rs[32..], sv)?;
            v.ed25519(pk, &rs, d.bytes())
        }
        (Material::Ed25519(pk), 27, [raw]) => {
            v.ed25519(pk, (*raw).try_into().map_err(|_| Refusal::Malformed)?, d.bytes())
        }
        _ => return Err(Refusal::Algorithm),
    };
    match good {
        true => Ok(Verified { fingerprint: key.fingerprint, hash: s.hash }),
        false => Err(Refusal::BadSignature),
    }
}
