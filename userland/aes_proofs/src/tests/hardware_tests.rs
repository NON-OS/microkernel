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


//! The AES-NI path and the computed S-box agree, on the published vectors
//! and on a run of keys and blocks, for both key sizes. One test, because
//! the choice of path is process-wide and the two halves must not interleave.

use crate::hex::hex;
use nonos_aes::{hardware, Aes128, Aes256, Ctr128Be, Ctr256Be};

fn block(text: &str) -> [u8; 16] {
    hex(text).try_into().unwrap()
}

fn vectors() {
    // FIPS 197 appendices B and C.1 (AES-128) and C.3 (AES-256).
    let mut b = block("3243f6a8885a308d313198a2e0370734");
    Aes128::new(&block("2b7e151628aed2a6abf7158809cf4f3c")).encrypt_block(&mut b);
    assert_eq!(b.to_vec(), hex("3925841d02dc09fbdc118597196a0b32"));
    let mut b = block("00112233445566778899aabbccddeeff");
    Aes128::new(&block("000102030405060708090a0b0c0d0e0f")).encrypt_block(&mut b);
    assert_eq!(b.to_vec(), hex("69c4e0d86a7b0430d8cdb78070b4c55a"));
    let mut b = block("00112233445566778899aabbccddeeff");
    let key: [u8; 32] = hex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").try_into().unwrap();
    Aes256::new(&key).encrypt_block(&mut b);
    assert_eq!(b.to_vec(), hex("8ea2b7ca516745bfeafc49904b496089"));
}

fn keystreams(seed: u64) -> std::vec::Vec<u8> {
    let mut x = seed;
    let mut next = || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x as u8
    };
    let mut out = std::vec::Vec::new();
    for _ in 0..64 {
        let k16: [u8; 16] = core::array::from_fn(|_| next());
        let k32: [u8; 32] = core::array::from_fn(|_| next());
        let mut a = std::vec![0u8; 509];
        Ctr128Be::new(&k16).apply(&mut a);
        let mut b = std::vec![0u8; 509];
        Ctr256Be::new(&k32).apply(&mut b);
        out.extend_from_slice(&a);
        out.extend_from_slice(&b);
    }
    out
}

#[test]
fn hardware_and_software_give_the_same_cipher() {
    hardware::force_software(true);
    assert!(!hardware::in_use());
    vectors();
    let soft = keystreams(0xAE5_0001);

    hardware::force_software(false);
    let hard_present = hardware::in_use();
    vectors();
    let hard = keystreams(0xAE5_0001);
    assert_eq!(soft, hard, "the two paths disagree");
    if !hard_present {
        std::println!("note: this CPU has no AES-NI; only the software path ran");
    }
}
