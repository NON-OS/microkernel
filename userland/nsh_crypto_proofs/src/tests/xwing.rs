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

//! The X-Wing exchange against the specification's three vectors: the key
//! pair from the seed, the ciphertext and secret from the encapsulation
//! randomness, and decapsulation back to the same secret.

use crate::{arr, de};
use nsh_crypto::{encapsulate, ClientKem, CryptoError, Secret, EK_LEN};

#[test]
fn every_published_vector() {
    let text = include_str!("../../vectors/xwing.txt");
    let mut n = 0;
    for f in text.lines().filter(|l| !l.starts_with('#')).map(|l| l.split(' ').collect::<Vec<_>>())
    {
        let (kem, ek) = ClientKem::new(Secret::new(arr(f[0])));
        assert_eq!(ek.to_vec(), de(f[3]), "public key");
        let (ct, ss) = encapsulate(&ek, Secret::new(arr(f[1]))).unwrap();
        assert_eq!((ct.to_vec(), ss.expose().to_vec()), (de(f[4]), de(f[2])));
        assert_eq!(kem.decapsulate(&ct).expose().to_vec(), de(f[2]));
        n += 1;
    }
    assert_eq!(n, 3);
}

#[test]
fn a_changed_ciphertext_gives_another_secret() {
    let (kem, ek) = ClientKem::new(Secret::new([7; 32]));
    let (ct, ss) = encapsulate(&ek, Secret::new([9; 64])).unwrap();
    let (kem2, _) = ClientKem::new(Secret::new([7; 32]));
    for (k, i) in [(kem, 1119), (kem2, 0)] {
        let mut bad = ct;
        bad[i] ^= 1;
        assert_ne!(k.decapsulate(&bad).expose(), ss.expose(), "byte {i}");
    }
}

#[test]
fn a_malformed_encapsulation_key_is_refused() {
    let bad = [0xFF; EK_LEN];
    assert!(matches!(encapsulate(&bad, Secret::new([1; 64])), Err(CryptoError::BadKey)));
}
