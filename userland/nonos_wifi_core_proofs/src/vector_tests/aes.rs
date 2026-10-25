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
//! AES-128 both ways (FIPS-197 appendix C.1), AES-CCM (RFC 3610 packet
//! vector 1, with a tamper check) and AES key unwrap (RFC 3394 section 4.1,
//! with a wrong KEK).

use nonos_wifi_core::ccmp::aes::Aes128;
use nonos_wifi_core::ccmp::ccm::{ccm_decrypt, ccm_encrypt};
use nonos_wifi_core::ccmp::keywrap::aes_unwrap;

use super::sha1::h;

fn a16(s: &str) -> [u8; 16] {
    h(s).try_into().unwrap()
}

#[test]
fn aes128_matches_fips197_both_ways() {
    let aes = Aes128::new(&a16("000102030405060708090a0b0c0d0e0f"));
    let mut b = a16("00112233445566778899aabbccddeeff");
    aes.encrypt_block(&mut b);
    assert_eq!(b, a16("69c4e0d86a7b0430d8cdb78070b4c55a"));
    aes.decrypt_block(&mut b);
    assert_eq!(b, a16("00112233445566778899aabbccddeeff"));
}

#[test]
fn aes_ccm_matches_rfc3610_packet_vector_1() {
    let key = a16("c0c1c2c3c4c5c6c7c8c9cacbcccdcecf");
    let nonce: [u8; 13] = h("00000003020100a0a1a2a3a4a5").try_into().unwrap();
    let (aad, payload) =
        (h("0001020304050607"), h("08090a0b0c0d0e0f101112131415161718191a1b1c1d1e"));
    let want = h("588c979a61c663d2f066d0c2c0f989806d5f6b61dac38417e8d12cfdf926e0");
    let mut out = [0u8; 31];
    assert_eq!(ccm_encrypt(&key, &nonce, &aad, &payload, &mut out), Some(31));
    assert_eq!(out.to_vec(), want);
    let mut back = [0u8; 23];
    assert_eq!(ccm_decrypt(&key, &nonce, &aad, &out, &mut back), Some(23));
    assert_eq!(back.to_vec(), payload);
    out[3] ^= 1;
    assert_eq!(ccm_decrypt(&key, &nonce, &aad, &out, &mut back), None, "tamper rejected");
}

#[test]
fn aes_key_unwrap_matches_rfc3394() {
    let mut kek = a16("000102030405060708090a0b0c0d0e0f");
    let wrapped = h("1fa68b0a8112b447aef34bd8fb5a7b829d3e862371d2cfe5");
    let mut out = [0u8; 16];
    assert_eq!(aes_unwrap(&kek, &wrapped, &mut out), Some(16));
    assert_eq!(out, a16("00112233445566778899aabbccddeeff"));
    kek[0] ^= 1;
    assert_eq!(aes_unwrap(&kek, &wrapped, &mut out), None, "a wrong KEK is refused");
}
