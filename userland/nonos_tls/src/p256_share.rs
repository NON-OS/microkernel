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

//! The secp256r1 ECDHE share, for servers that take P-256 and not X25519.

use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::{FieldBytes, PublicKey, SecretKey};

/*
 * A scalar from the random source is refused when it is zero or not below
 * the group order, and a fresh one is drawn. One refusal has odds near
 * 2^-32, so eight draws all refused means the source is broken.
 */
pub fn generate() -> Option<([u8; 32], [u8; 65])> {
    for _ in 0..8 {
        let mut scalar = [0u8; 32];
        if nonos_libc::crypto_random(scalar.as_mut_ptr(), scalar.len()) != scalar.len() as i64 {
            return None;
        }
        if let Ok(secret) = SecretKey::from_bytes(&FieldBytes::from(scalar)) {
            let point = secret.public_key().to_encoded_point(false);
            let mut public = [0u8; 65];
            public.copy_from_slice(point.as_bytes());
            return Some((scalar, public));
        }
    }
    None
}

/*
 * RFC 8446 7.4.2: the shared secret is the x coordinate of the product, 32
 * bytes for P-256. The peer's point must decode onto the curve and not be
 * the identity; from_sec1_bytes refuses both.
 */
pub fn shared(private: &[u8; 32], peer: &[u8; 65]) -> Option<[u8; 32]> {
    let secret = SecretKey::from_bytes(&FieldBytes::from(*private)).ok()?;
    let public = PublicKey::from_sec1_bytes(peer).ok()?;
    let product = p256::ecdh::diffie_hellman(secret.to_nonzero_scalar(), public.as_affine());
    let mut out = [0u8; 32];
    out.copy_from_slice(product.raw_secret_bytes());
    Some(out)
}
