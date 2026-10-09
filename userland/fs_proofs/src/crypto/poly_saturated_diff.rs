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

//! Poly1305 differential cases the random ones rarely reach: every limb of
//! the key and of the message saturated, and seals of the sector's shape
//! (the cryptoblock AAD prefix and LBA, 484 bytes of body).

use super::chacha20poly1305 as now;
use super::prior_chacha20poly1305 as prior;

#[test]
fn mac_matches_prior_at_saturated_inputs() {
    for key_byte in [0x00u8, 0x01, 0x80, 0xfe, 0xff] {
        for msg_byte in [0x00u8, 0x01, 0xfe, 0xff] {
            let key = [key_byte; 32];
            let msg = [msg_byte; 160];
            for len in 0..=msg.len() {
                let (a, b) =
                    (now::poly1305_mac(&msg[..len], &key), prior::poly1305_mac(&msg[..len], &key));
                assert_eq!(a, b, "key {key_byte:#x} msg {msg_byte:#x} len {len}");
            }
        }
    }
}

#[test]
fn sector_shaped_seals_match_prior() {
    let mut seed = 0x0123_4567_89ab_cdefu64;
    let mut next = || {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (seed >> 33) as u8
    };
    for lba in 0..3000u64 {
        let key: [u8; 32] = core::array::from_fn(|_| next());
        let nonce: [u8; 12] = core::array::from_fn(|_| next());
        let body: Vec<u8> = (0..484).map(|_| next()).collect();
        let mut aad = *b"NONOSCRYPTBLK001\0\0\0\0\0\0\0\0";
        aad[16..].copy_from_slice(&lba.to_le_bytes());
        let a = now::aead_encrypt(&key, &nonce, &aad, &body).unwrap();
        assert_eq!(a, prior::aead_encrypt(&key, &nonce, &aad, &body), "lba {lba}");
    }
}
