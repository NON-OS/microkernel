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

//! The deterministic randomness the test keys and signatures draw on, and
//! the key each seed makes.

use super::keys::TestKey;
use nonos_ed25519::KeyPair;
use nonos_hash::{sha256, Sha512};
use nonos_mldsa65::keypair;

/// A stream of bytes from SHA-512 of a seed and a counter.
pub fn stream(seed: &[u8]) -> impl FnMut(&mut [u8]) -> bool + '_ {
    let mut n = 0u64;
    move |out: &mut [u8]| {
        for chunk in out.chunks_mut(64) {
            let mut h = Sha512::new();
            h.update(seed);
            h.update(&n.to_le_bytes());
            n += 1;
            chunk.copy_from_slice(&h.finalize()[..chunk.len()]);
        }
        true
    }
}

pub fn make(i: u8) -> TestKey {
    let seed = [b'n', b's', b'h', b'-', b'r', b'o', b'l', b'e', i];
    let ed = KeyPair::from_seed(sha256(&seed));
    let (ml_pk, ml_sk) = keypair(&mut stream(&seed)).expect("ML-DSA-65 key pair");
    TestKey { ed, ml_pk, ml_sk }
}
