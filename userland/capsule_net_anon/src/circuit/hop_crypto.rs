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


//! The two kinds of hop crypto a circuit carries.
//!
//! Every relay hop runs SHA-1 and AES-128. The last hop of a rendezvous
//! circuit is the onion service itself, and relay_crypto_init with is_hs_v3
//! gives it SHA3-256 and AES-256 instead. The cell format and the four
//! digest bytes are the same; a SENDME names the first twenty bytes of the
//! digest either way (relay_crypto_record_sendme_digest).

use nonos_aes::{Ctr128Be, Ctr256Be};

use crate::crypto::keccak::Sha3_256;
use crate::crypto::sha1::Sha1;

/// A running digest over the cells one way on one hop.
#[derive(Clone)]
pub enum RunningDigest {
    Sha1(Sha1),
    Sha3(Sha3_256),
}

impl RunningDigest {
    pub fn update(&mut self, data: &[u8]) {
        match self {
            RunningDigest::Sha1(h) => h.update(data),
            RunningDigest::Sha3(h) => h.update(data),
        }
    }

    /// The first twenty bytes of the digest so far, state kept.
    pub fn peek(&self) -> [u8; 20] {
        match self {
            RunningDigest::Sha1(h) => h.peek(),
            RunningDigest::Sha3(h) => {
                let mut out = [0u8; 20];
                out.copy_from_slice(&h.peek()[..20]);
                out
            }
        }
    }
}

/// The counter mode keystream one way on one hop.
pub enum Keystream {
    Aes128(Ctr128Be),
    Aes256(Ctr256Be),
}

impl Keystream {
    pub fn apply(&mut self, data: &mut [u8]) {
        match self {
            Keystream::Aes128(k) => k.apply(data),
            Keystream::Aes256(k) => k.apply(data),
        }
    }
}
