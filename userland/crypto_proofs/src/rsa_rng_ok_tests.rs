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

//! With the generator there, RSA and Miller-Rabin work as before: a prime
//! past 64 bits passes, a composite fails, and OAEP encrypts.

use crate::crypto::asymmetric::rsa::{create_public_key, oaep_encrypt, BigUint};
use crate::keygen_tests::TURN;
use crate::rsa_rng_tests::m127;

#[test]
fn with_the_generator_rsa_works() {
    let _turn = TURN.lock().unwrap_or_else(|p| p.into_inner());
    assert!(m127().is_probably_prime(16));
    let composite = BigUint::from_hex("7ffffffffffffffffffffffffffffffd").unwrap();
    assert!(!composite.is_probably_prime(16));
    let (lo, hi) = (BigUint::from_u64(2), m127());
    let a = BigUint::random_range(&lo, &hi).expect("drawn");
    assert!(a >= lo && a < hi);
    // A range of two: every draw is 2 or 3, never the upper end.
    let (two, four) = (BigUint::from_u64(2), BigUint::from_u64(4));
    for _ in 0..256 {
        assert!(BigUint::random_range(&two, &four).expect("drawn") < four);
    }
    let key = create_public_key(vec![0xC5; 256], vec![1, 0, 1]);
    assert_eq!(oaep_encrypt(&key, b"nsh").map(|c| c.len()), Ok(256));
}
