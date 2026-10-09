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


//! The TLS 1.2 PRF (RFC 5246 5): P_hash over HMAC, SHA-256 for the
//! ChaCha20 suite and SHA-384 for the AES-256-GCM one.

extern crate alloc;

use alloc::vec::Vec;

use crate::crypto::hmac_sha256;
use crate::crypto::sha384::{hmac_sha384, sha384};

use super::constants::SUITE_AES256_GCM;
use super::error::Tls12Error;

/// The hash a suite runs its PRF and transcript over.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hash {
    Sha256,
    Sha384,
}

impl Hash {
    pub fn of(suite: u16) -> Self {
        if suite == SUITE_AES256_GCM {
            Self::Sha384
        } else {
            Self::Sha256
        }
    }

    fn hmac(self, key: &[u8], data: &[u8]) -> Result<Vec<u8>, Tls12Error> {
        match self {
            Self::Sha256 => {
                let mut out = [0u8; 32];
                hmac_sha256(key, data, &mut out).map_err(|_| Tls12Error::Crypto)?;
                Ok(out.to_vec())
            }
            Self::Sha384 => Ok(hmac_sha384(key, &[data]).to_vec()),
        }
    }

    /// The transcript hash.
    pub fn digest(self, data: &[u8]) -> Result<Vec<u8>, Tls12Error> {
        match self {
            Self::Sha256 => Ok(crate::crypto::sha256(data).map_err(|_| Tls12Error::Crypto)?.to_vec()),
            Self::Sha384 => Ok(sha384(&[data]).to_vec()),
        }
    }
}

/// PRF(secret, label, seed), `out.len()` bytes.
pub fn prf(hash: Hash, secret: &[u8], label: &[u8], seed: &[u8], out: &mut [u8]) -> Result<(), Tls12Error> {
    let mut label_seed = Vec::with_capacity(label.len() + seed.len());
    label_seed.extend_from_slice(label);
    label_seed.extend_from_slice(seed);
    // A(1) = HMAC(secret, label_seed); each block is HMAC(secret, A(i) | label_seed).
    let mut a = hash.hmac(secret, &label_seed)?;
    let mut at = 0usize;
    while at < out.len() {
        let mut input = a.clone();
        input.extend_from_slice(&label_seed);
        let mut block = hash.hmac(secret, &input)?;
        let take = block.len().min(out.len() - at);
        out[at..at + take].copy_from_slice(&block[..take]);
        at += take;
        crate::crypto::wipe::wipe(&mut block);
        crate::crypto::wipe::wipe(&mut input);
        let next = hash.hmac(secret, &a)?;
        crate::crypto::wipe::wipe(&mut a);
        a = next;
    }
    crate::crypto::wipe::wipe(&mut a);
    Ok(())
}
