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


//! The service's identity, blinded for a period, from its secret key
//! (ed25519_donna_blind_secret_key and hs_build_blinded_keypair), with
//! curve25519-dalek rather than the client's own field arithmetic.

use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
use ed25519_dalek::hazmat::{raw_sign, ExpandedSecretKey};
use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha512};
use sha3::Sha3_256;

const BASEPOINT: &[u8] = b"(15112221349535400772501151409588531511454012693041857206046113283949847762202, 46316835694926478169428394003475163141307993866256225615783033603165251855960)";

/// The blinded secret key and the public key it signs as.
pub struct Blinded {
    pub key: ExpandedSecretKey,
    pub public: [u8; 32],
}

fn clamp(mut b: [u8; 32]) -> [u8; 32] {
    b[0] &= 248;
    b[31] &= 63;
    b[31] |= 64;
    b
}

/// The identity public key for `seed`.
pub fn identity(seed: &[u8; 32]) -> [u8; 32] {
    ed25519_dalek::SigningKey::from_bytes(seed).verifying_key().to_bytes()
}

/// Blind the identity with secret seed `seed` for `period` of `length`
/// minutes.
pub fn blind(seed: &[u8; 32], period: u64, length: u64) -> Blinded {
    let expanded: [u8; 64] = Sha512::digest(seed).into();
    let a = Scalar::from_bytes_mod_order(clamp(expanded[..32].try_into().unwrap()));
    let public = identity(seed);

    let mut nonce = b"key-blind".to_vec();
    nonce.extend_from_slice(&period.to_be_bytes());
    nonce.extend_from_slice(&length.to_be_bytes());
    let mut hasher = Sha3_256::new();
    for part in [&b"Derive temporary signing key\0"[..], &public, BASEPOINT, &nonce] {
        hasher.update(part);
    }
    let h = Scalar::from_bytes_mod_order(clamp(hasher.finalize().into()));
    let scalar = h * a;

    let mut prefix_hash = Sha512::new();
    prefix_hash.update(b"Derive temporary signing key hash input");
    prefix_hash.update(&expanded[32..]);
    let hash_prefix: [u8; 32] = prefix_hash.finalize()[..32].try_into().unwrap();

    let blinded_public = (scalar * ED25519_BASEPOINT_POINT).compress().to_bytes();
    Blinded { key: ExpandedSecretKey { scalar, hash_prefix }, public: blinded_public }
}

impl Blinded {
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        let verifying = VerifyingKey::from_bytes(&self.public).unwrap();
        raw_sign::<Sha512>(&self.key, message, &verifying).to_bytes()
    }
}
