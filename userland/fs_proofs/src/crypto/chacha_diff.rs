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

//! The shipping ChaCha20-Poly1305 against the prior implementation kept in
//! `prior_chacha20poly1305`: every output must match byte for byte over
//! random keys, nonces, counters, AAD and message lengths.

use super::chacha20poly1305 as now;
use super::prior_chacha20poly1305 as prior;
use super::test_input::Rng;

const LENS: [usize; 20] =
    [0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 484, 511, 512, 513, 544, 4096];

#[test]
fn block_matches_prior_for_random_counters() {
    let mut rng = Rng(1);
    for i in 0..2000u32 {
        let (key, nonce) = (rng.bytes::<32>(), rng.bytes::<12>());
        let counter = [0, 1, u32::MAX, rng.next() as u32][i as usize % 4];
        let (mut a, mut b) = ([0u8; 64], [0u8; 64]);
        now::chacha20_block(&key, &nonce, counter, &mut a);
        prior::chacha20_block(&key, &nonce, counter, &mut b);
        assert_eq!(a, b, "counter {counter}");
    }
}

#[test]
fn mac_and_aead_match_prior_over_lengths() {
    let mut rng = Rng(2);
    let random = (0..300).map(|i| if i % 2 == 0 { i % 97 } else { rng.next() as usize % 2048 });
    let lens: Vec<usize> = LENS.into_iter().chain(random.collect::<Vec<_>>()).collect();
    for (i, len) in lens.into_iter().enumerate() {
        let (key, nonce) = (rng.bytes::<32>(), rng.bytes::<12>());
        let aad_len = [0, 1, 12, 15, 16, 17, 24, rng.next() as usize % 64][i % 8];
        let (aad, msg) = (rng.vec(aad_len), rng.vec(len));
        assert_eq!(now::poly1305_mac(&msg, &key), prior::poly1305_mac(&msg, &key), "len {len}");
        let sealed = now::aead_encrypt(&key, &nonce, &aad, &msg).unwrap();
        assert_eq!(sealed, prior::aead_encrypt(&key, &nonce, &aad, &msg), "len {len}");
        assert_eq!(now::aead_decrypt(&key, &nonce, &aad, &sealed).unwrap(), msg);
        let mut buf = msg.clone();
        buf.resize(len + 16, 0);
        assert_eq!(now::aead_encrypt_in_place(&key, &nonce, &aad, &mut buf, len), Ok(len + 16));
        assert_eq!(buf, sealed, "in place, len {len}");
        assert_eq!(now::aead_decrypt_in_place(&key, &nonce, &aad, &mut buf, len + 16), Ok(len));
        assert_eq!(buf[..len], msg[..]);
    }
}
