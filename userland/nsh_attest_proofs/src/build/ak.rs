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

//! An attestation key the proofs hold: ECDSA P-256 over the checker's own
//! arithmetic, with a nonce derived from the key and the digest.

use nonos_hash::sha256;
use nsh_attest::p256::{AffinePoint, Scalar};

pub struct Key {
    d: Scalar,
}

fn g() -> nsh_attest::p256::ProjectivePoint {
    AffinePoint::generator().to_projective()
}

impl Key {
    pub fn new(tag: u8) -> Self {
        Key { d: Scalar::from_bytes_reduce(&sha256(&[b'a', b'k', tag])) }
    }

    /// The uncompressed point, x then y.
    pub fn public(&self) -> [u8; 64] {
        let p = g().mul(&self.d).to_affine().to_uncompressed();
        p[1..].try_into().expect("64 bytes")
    }

    /// r and s over `digest`; `salt` picks another nonce.
    pub fn sign(&self, digest: &[u8; 32], salt: u32) -> ([u8; 32], [u8; 32]) {
        let seed = [&self.d.to_bytes()[..], digest, &salt.to_be_bytes()].concat();
        let k = Scalar::from_bytes_reduce(&sha256(&seed));
        let r = Scalar::from_bytes_reduce(&g().mul(&k).to_affine().x.to_bytes());
        let z = Scalar::from_bytes_reduce(digest);
        let s = k.invert().expect("k is not zero").mul(&z.add(&r.mul(&self.d)));
        (r.to_bytes(), s.to_bytes())
    }
}
