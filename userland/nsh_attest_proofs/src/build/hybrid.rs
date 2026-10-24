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

//! Hybrid keys, Ed25519 and ML-DSA-65, made from a tag so every run uses
//! the same keys, and the labelled signatures nsh/1 makes with them.

use nonos_ed25519::KeyPair;
use nonos_hash::sha256;
use nonos_mldsa65::SecretKey;

pub struct Hybrid {
    pub ed: KeyPair,
    pub ml_public: Box<[u8; 1952]>,
    ml_secret: SecretKey,
    tag: u8,
}

/// Randomness for ML-DSA, from a counter: fixed, so the keys are.
fn source(tag: u8) -> impl FnMut(&mut [u8]) -> bool {
    let mut n = 0u64;
    move |out: &mut [u8]| {
        for chunk in out.chunks_mut(32) {
            n += 1;
            let block = sha256(&[&[tag][..], &n.to_be_bytes()].concat());
            chunk.copy_from_slice(&block[..chunk.len()]);
        }
        true
    }
}

impl Hybrid {
    pub fn new(tag: u8) -> Self {
        let ed = KeyPair::from_seed(sha256(&[b'e', b'd', tag]));
        let (pk, ml_secret) = nonos_mldsa65::keypair(&mut source(tag)).expect("a key pair");
        Hybrid { ed, ml_public: Box::new(pk), ml_secret, tag }
    }

    /// Both halves over `digest`, Ed25519 then ML-DSA-65.
    pub fn sign(&self, digest: &[u8]) -> Vec<u8> {
        let mut out = nonos_ed25519::sign(&self.ed, digest).to_bytes().to_vec();
        let ml = nonos_mldsa65::sign(&self.ml_secret, digest, &mut source(self.tag ^ 0x80));
        out.extend(ml.expect("a signature"));
        out
    }

    pub fn authority(&self) -> nsh_attest::Authority {
        nsh_attest::Authority::new(self.ed.public, self.ml_public.clone())
    }
}
