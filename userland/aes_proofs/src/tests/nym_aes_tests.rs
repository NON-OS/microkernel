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

//! The capsule_net_nym ciphers against FIPS 197 and SP 800-38A.

use super::aes_tests::block;
use super::nym_aes::aes::{Aes128, Aes256, Ctr64Be, BLOCK_BYTES, KEY_BYTES, KEY_BYTES_256};
use crate::hex::hex;

// FIPS 197, appendix C.1.
#[test]
fn nym_aes128_fips197_appendix_c1() {
    let key: [u8; KEY_BYTES] = block("000102030405060708090a0b0c0d0e0f");
    let mut data: [u8; BLOCK_BYTES] = block("00112233445566778899aabbccddeeff");
    Aes128::new(&key).encrypt_block(&mut data);
    assert_eq!(data.to_vec(), hex("69c4e0d86a7b0430d8cdb78070b4c55a"));
}

// FIPS 197, appendix C.3.
#[test]
fn nym_aes256_fips197_appendix_c3() {
    let mut key = [0u8; KEY_BYTES_256];
    key.copy_from_slice(&hex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"));
    let mut data = block("00112233445566778899aabbccddeeff");
    Aes256::new(&key).encrypt_block(&mut data);
    assert_eq!(data.to_vec(), hex("8ea2b7ca516745bfeafc49904b496089"));
}

// NIST SP 800-38A, F.5.1: CTR-AES128, non-zero initial counter block.
#[test]
fn nym_ctr64_sp800_38a_f51() {
    let key = block("2b7e151628aed2a6abf7158809cf4f3c");
    let iv = block("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let mut data = hex(super::ctr_vectors::F51_PLAINTEXT);
    Ctr64Be::new(&key, &iv).apply(&mut data);
    assert_eq!(data, hex(super::ctr_vectors::F51_CIPHERTEXT));
    let mut stream = vec![0u8; 16];
    Ctr64Be::new(&key, &iv).keystream(&mut stream);
    let mut expected = iv;
    Aes128::new(&key).encrypt_block(&mut expected);
    assert_eq!(stream, expected.to_vec());
}
