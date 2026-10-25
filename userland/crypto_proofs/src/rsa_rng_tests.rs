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

//! RSA draws its prime candidates, its PSS salt, its OAEP seed and its
//! padding only from the seeded RNG, and Miller-Rabin draws its witnesses
//! there too: with the generator taken away, each refuses.

use crate::crypto::asymmetric::rsa::{
    create_public_key, encrypt, generate_keypair_with_bits, oaep_encrypt, sign_pss, BigUint,
    RsaPrivateKey,
};
use crate::crypto::error::CryptoError;
use crate::crypto::rng::set_unavailable;
use crate::keygen_tests::TURN;

/// 2^127 - 1, a prime past 64 bits, so its test draws random witnesses.
pub(crate) fn m127() -> BigUint {
    BigUint::from_hex("7fffffffffffffffffffffffffffffff").unwrap()
}

#[test]
fn with_no_generator_rsa_refuses() {
    let _turn = TURN.lock().unwrap_or_else(|p| p.into_inner());
    let key = create_public_key(vec![0xC5; 256], vec![1, 0, 1]);
    set_unavailable(true);
    let pair = generate_keypair_with_bits(1024).map(|_| ());
    let oaep = oaep_encrypt(&key, b"nsh").map(|_| ());
    let pkcs1 = encrypt(b"nsh", &key).map(|_| ());
    let prime = m127().is_probably_prime(16);
    let range = BigUint::random_range(&BigUint::from_u64(2), &m127());
    let one = || BigUint::from_u64(1);
    let (n, e, d) = (BigUint::from_bytes_be(&[0xC5; 256]), one(), one());
    let sk = RsaPrivateKey {
        n,
        e,
        d,
        p: one(),
        q: one(),
        dp: one(),
        dq: one(),
        qinv: one(),
        bits: 2048,
    };
    let pss = sign_pss(b"nsh", &sk).map(|_| ());
    set_unavailable(false);
    assert_eq!(pair, Err(CryptoError::InsufficientEntropy));
    assert_eq!(oaep, Err(CryptoError::InsufficientEntropy));
    assert_eq!(pkcs1, Err("no randomness"));
    assert!(!prime, "a test that cannot draw witnesses proves nothing");
    assert!(range.is_none());
    assert_eq!(pss, Err("no randomness"), "the salt is drawn before the key is used");
}
