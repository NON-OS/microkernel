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

//! The heap-free opener against sectors the kernel's own AEAD sealed.

use super::constants::{NONCE_BYTES, PLAIN_BLOCK_BYTES};
use super::opener::open_sealed;
use super::seal_fixture::{aad, fill, sealed};
use crate::crypto::chacha20poly1305::aead_decrypt;

#[test]
fn a_sector_the_kernel_sealed_opens_to_its_plaintext() {
    let mut seed = 7;
    for round in 0..200u64 {
        let (mut key, mut plain) = ([0u8; 32], [0u8; PLAIN_BLOCK_BYTES]);
        fill(&mut seed, &mut key);
        fill(&mut seed, &mut plain);
        let lba = round.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> (round % 64);
        let sector = sealed(&mut seed, &key, lba, &plain);
        let mut out = [0x5a; PLAIN_BLOCK_BYTES];
        assert!(open_sealed(&key, lba, &sector, &mut out), "round {round}");
        assert_eq!(out, plain);
        let nonce: [u8; NONCE_BYTES] = sector[..NONCE_BYTES].try_into().unwrap();
        let theirs = aead_decrypt(&key, &nonce, &aad(lba), &sector[NONCE_BYTES..]).unwrap();
        assert_eq!(theirs, plain);
    }
}
