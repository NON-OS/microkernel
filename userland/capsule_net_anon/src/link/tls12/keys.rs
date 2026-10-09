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


//! The key agreement and the secrets derived from it (RFC 8422 5.10,
//! RFC 5246 6.3, 7.4.9, 8.1, RFC 7627 4).

extern crate alloc;

use alloc::vec::Vec;

use nonos_libc::crypto_random;
use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::{FieldBytes, PublicKey, SecretKey};

use super::constants::SUITE_CHACHA20;
use super::error::Tls12Error;
use super::prf::{prf, Hash};
use super::record::Direction;

/// A secret that is wiped when it goes out of scope, on every path.
pub struct Secret<const N: usize>(pub [u8; N]);

impl<const N: usize> Drop for Secret<N> {
    fn drop(&mut self) {
        crate::crypto::wipe::wipe(&mut self.0);
    }
}

/// Our ECDHE share: the scalar, wiped when dropped, and the point we send.
pub struct Share {
    scalar: [u8; 32],
    pub public: [u8; 65],
}

impl Drop for Share {
    fn drop(&mut self) {
        crate::crypto::wipe::wipe(&mut self.scalar);
    }
}

impl Share {
    /// A fresh scalar. One that is zero or not below the group order is
    /// drawn again; eight refusals in a row mean the source is broken.
    pub fn generate() -> Result<Self, Tls12Error> {
        for _ in 0..8 {
            let mut scalar = [0u8; 32];
            if crypto_random(scalar.as_mut_ptr(), scalar.len()) != scalar.len() as i64 {
                return Err(Tls12Error::Crypto);
            }
            if let Ok(secret) = SecretKey::from_bytes(&FieldBytes::from(scalar)) {
                let point = secret.public_key().to_encoded_point(false);
                let mut public = [0u8; 65];
                public.copy_from_slice(point.as_bytes());
                return Ok(Self { scalar, public });
            }
            crate::crypto::wipe::wipe(&mut scalar);
        }
        Err(Tls12Error::Crypto)
    }

    /// The premaster secret: the x coordinate of our scalar times the
    /// server's point. A point off the curve, or the identity, is refused.
    pub fn agree(&self, peer: &[u8; 65]) -> Result<Secret<32>, Tls12Error> {
        let secret = SecretKey::from_bytes(&FieldBytes::from(self.scalar)).map_err(|_| Tls12Error::Crypto)?;
        let public = PublicKey::from_sec1_bytes(peer).map_err(|_| Tls12Error::Crypto)?;
        let product = p256::ecdh::diffie_hellman(secret.to_nonzero_scalar(), public.as_affine());
        let mut out = Secret([0u8; 32]);
        out.0.copy_from_slice(product.raw_secret_bytes());
        Ok(out)
    }
}

/// The master secret. With the extended master secret (RFC 7627) it is
/// bound to the hash of the whole handshake through ClientKeyExchange;
/// without it, to the two randoms only.
pub fn master_secret(hash: Hash, premaster: &[u8; 32], ems: Option<&[u8]>, randoms: &[u8; 64]) -> Result<Secret<48>, Tls12Error> {
    let mut out = Secret([0u8; 48]);
    match ems {
        Some(session_hash) => prf(hash, premaster, b"extended master secret", session_hash, &mut out.0)?,
        None => prf(hash, premaster, b"master secret", randoms, &mut out.0)?,
    }
    Ok(out)
}

/// The two directions' keys from the master secret, client first. The
/// key block seed is the server random then the client random.
pub fn directions(suite: u16, master: &[u8; 48], client_random: &[u8; 32], server_random: &[u8; 32]) -> Result<(Direction, Direction), Tls12Error> {
    let iv_len = if suite == SUITE_CHACHA20 { 12 } else { 4 };
    let mut seed = [0u8; 64];
    seed[..32].copy_from_slice(server_random);
    seed[32..].copy_from_slice(client_random);
    let mut block = alloc::vec![0u8; 64 + 2 * iv_len];
    prf(Hash::of(suite), master, b"key expansion", &seed, &mut block)?;
    let mut client = Direction { suite, key: [0; 32], iv: [0; 12], seq: 0 };
    let mut server = Direction { suite, key: [0; 32], iv: [0; 12], seq: 0 };
    client.key.copy_from_slice(&block[..32]);
    server.key.copy_from_slice(&block[32..64]);
    client.iv[..iv_len].copy_from_slice(&block[64..64 + iv_len]);
    server.iv[..iv_len].copy_from_slice(&block[64 + iv_len..64 + 2 * iv_len]);
    crate::crypto::wipe::wipe(&mut block);
    Ok((client, server))
}

/// A Finished message's verify_data, for `label` ("client finished" or
/// "server finished") over the transcript so far.
pub fn verify_data(hash: Hash, master: &[u8; 48], label: &[u8], transcript: &[u8]) -> Result<[u8; 12], Tls12Error> {
    let digest: Vec<u8> = hash.digest(transcript)?;
    let mut out = [0u8; 12];
    prf(hash, master, label, &digest, &mut out)?;
    Ok(out)
}
