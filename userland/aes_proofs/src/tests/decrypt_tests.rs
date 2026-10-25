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

//! The inverse cipher: FIPS 197's two worked examples run backwards, and
//! a round trip over blocks that put every byte value through every
//! position of the state.

use super::aes_tests::block;
use crate::hex::hex;
use nonos_aes::Aes128;

// FIPS 197, appendix B and C.1, from ciphertext back to plaintext.
#[test]
fn fips197_examples_decrypt() {
    let cases = [
        (
            "2b7e151628aed2a6abf7158809cf4f3c",
            "3925841d02dc09fbdc118597196a0b32",
            "3243f6a8885a308d313198a2e0370734",
        ),
        (
            "000102030405060708090a0b0c0d0e0f",
            "69c4e0d86a7b0430d8cdb78070b4c55a",
            "00112233445566778899aabbccddeeff",
        ),
    ];
    for (key, ct, pt) in cases {
        let mut data = block(ct);
        Aes128::new(&block(key)).decrypt_block(&mut data);
        assert_eq!(data.to_vec(), hex(pt), "key {key}");
    }
}

// Block v is (v, v+1, ..., v+15) for v in 0..=255, so each of the 256
// byte values meets each state position on the way in and on the way out.
#[test]
fn decrypt_inverts_encrypt_for_every_byte() {
    let aes = Aes128::new(&block("000102030405060708090a0b0c0d0e0f"));
    for v in 0..=u8::MAX {
        let plain: [u8; 16] = core::array::from_fn(|i| v.wrapping_add(i as u8));
        let mut data = plain;
        aes.encrypt_block(&mut data);
        assert_ne!(data, plain);
        aes.decrypt_block(&mut data);
        assert_eq!(data, plain, "block {v}");
        aes.decrypt_block(&mut data);
        aes.encrypt_block(&mut data);
        assert_eq!(data, plain, "block {v}, the other way round");
    }
}
