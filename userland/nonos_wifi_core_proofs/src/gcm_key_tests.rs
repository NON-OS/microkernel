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

//! AES-256 against FIPS 197 appendix C.3, and GCM test cases 15 and 16 of McGrew and Viega (AES-256 under a real key):
//! 64 octets, then 60 octets with 20 octets of AAD.

use super::gcm_tests::{case, h};
use nonos_wifi_core::gcmp::aes256::Aes256;

const K: &str = "feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308";
const IV: &str = "cafebabefacedbaddecaf888";
const P: &str = "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a72\
                 1c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255";
const C: &str = "522dc1f099567d07f47f37a32a84427d643a8cdcbfe5c0c97598a2bd2555d1aa\
                 8cb08e48590dbb3da7b08b1056828838c5f61e6393ba7a0abcc9f662898015ad";

// Recalled, the code reproduces it; source not reachable here.
#[test]
fn gcm_case_15_four_blocks() {
    let (p, c) = (P.replace(' ', ""), C.replace(' ', ""));
    case(K, IV, "", &p, &c, "b094dac5d93471bdec1a502270e3cc6c");
}

// Recalled, the code reproduces it; source not reachable here.
#[test]
fn gcm_case_16_with_aad_and_a_partial_block() {
    let (p, c) = (P.replace(' ', ""), C.replace(' ', ""));
    let aad = "feedfacedeadbeeffeedfacedeadbeefabaddad2";
    case(K, IV, aad, &p[..120], &c[..120], "76fc6ece0f4e1768cddf8853bb2d551b");
}

// Recalled, the code reproduces it; source not reachable here.
#[test]
fn aes256_matches_fips197_c3() {
    let k: [u8; 32] =
        h("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").try_into().unwrap();
    let mut b: [u8; 16] = h("00112233445566778899aabbccddeeff").try_into().unwrap();
    Aes256::new(&k).encrypt_block(&mut b);
    assert_eq!(b.to_vec(), h("8ea2b7ca516745bfeafc49904b496089"));
}
