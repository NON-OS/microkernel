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

//! Key generation draws only from the seeded RNG: with the generator taken
//! away, Ed25519 and P-256 key pairs are refused, never made from weaker
//! bytes; with it back, the pairs they make sign and verify.

use crate::crypto::asymmetric::{ed25519, p256};
use crate::crypto::error::CryptoError;
use crate::crypto::rng::set_unavailable;
use std::sync::Mutex;

/// The generator is one global, so the tests that take it away take turns.
pub(crate) static TURN: Mutex<()> = Mutex::new(());

#[test]
fn with_no_generator_no_key_is_made() {
    let _turn = TURN.lock().unwrap_or_else(|p| p.into_inner());
    set_unavailable(true);
    let ed = ed25519::KeyPair::generate().map(|_| ());
    let p = p256::generate_keypair().map(|_| ());
    let ecdh = p256::p256_ecdh_keypair().map(|_| ());
    let pair = p256::P256KeyPair::generate().map(|_| ());
    set_unavailable(false);
    for got in [ed, p, ecdh, pair] {
        assert_eq!(got, Err(CryptoError::InsufficientEntropy));
    }
}

#[test]
fn with_the_generator_pairs_sign_and_verify() {
    let _turn = TURN.lock().unwrap_or_else(|p| p.into_inner());
    let kp = ed25519::KeyPair::generate().expect("drawn");
    let other = ed25519::KeyPair::generate().expect("drawn");
    assert_ne!(kp.public, other.public, "each key pair comes from its own draw");
    let sig = ed25519::sign(&kp, b"nsh");
    assert!(ed25519::verify(&kp.public, b"nsh", &sig));
    assert!(!ed25519::verify(&kp.public, b"nsi", &sig));
    let pair = p256::P256KeyPair::generate().expect("drawn");
    let sig = pair.sign(b"nsh").expect("signed");
    assert!(pair.verify(b"nsh", &sig) && !pair.verify(b"nsi", &sig));
    let (sk, pk) = p256::generate_keypair().expect("drawn");
    assert_eq!(p256::public_key_from_secret(&sk), Some(pk));
    let (a, b) = (p256::p256_ecdh_keypair().unwrap(), p256::p256_ecdh_keypair().unwrap());
    assert_ne!(a.0, b.0, "two draws differ");
    assert_eq!(p256::p256_ecdh(&a.0, &b.1), p256::p256_ecdh(&b.0, &a.1));
}
