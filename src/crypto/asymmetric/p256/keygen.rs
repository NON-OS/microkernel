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

//! Drawing a P-256 key pair. Each draw takes 32 bytes from the seeded RNG
//! and fails if the RNG cannot answer; nothing is made from weaker bytes.

use super::{AffinePoint, PublicKey, Scalar, SecretKey};
use crate::crypto::error::CryptoResult;

/// A secret key and its public key, as `0x04 || x || y`.
pub fn generate_keypair() -> CryptoResult<(SecretKey, PublicKey)> {
    let mut sk = [0u8; 32];
    loop {
        crate::crypto::random_api::get_bytes_secure(&mut sk)?;
        // Zero, or not below the group order: drawn again. Zero needs no check
        // of its own, since it makes the point at infinity, refused below.
        let Some(scalar) = Scalar::from_bytes(&sk) else {
            continue;
        };
        let point = AffinePoint::generator().to_projective().mul(&scalar).to_affine();
        if !point.infinity {
            let pair = (sk, point.to_uncompressed());
            crate::crypto::constant_time::secure_zero(&mut sk);
            return Ok(pair);
        }
    }
}
