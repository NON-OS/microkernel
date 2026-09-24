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

//! The low 64 bit counter mode Sphinx runs, against SP 800-38A and itself.

use super::aes_tests::block;
use super::ctr_vectors::{F51_CIPHERTEXT, F51_PLAINTEXT};
use nonos_aes::{Aes128, Ctr64Be};

// NIST SP 800-38A, F.5.1: CTR-AES128, non-zero initial counter block.
#[test]
fn ctr64_sp800_38a_f51() {
    let key = block("2b7e151628aed2a6abf7158809cf4f3c");
    let iv = block("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let mut data = crate::hex::hex(F51_PLAINTEXT);
    Ctr64Be::new(&key, &iv).apply(&mut data);
    assert_eq!(data, crate::hex::hex(F51_CIPHERTEXT));
}

/// The low half wraps to zero on its own; the nonce prefix in the high half
/// must not move, or the keystream is a 128 bit counter's and not Sphinx's.
#[test]
fn low_64_bits_wrap_without_touching_the_prefix() {
    let key = block("000102030405060708090a0b0c0d0e0f");
    let iv = block("0123456789abcdefffffffffffffffff");
    let mut stream = vec![0u8; 32];
    Ctr64Be::new(&key, &iv).apply(&mut stream);
    let cipher = Aes128::new(&key);
    let mut first = iv;
    cipher.encrypt_block(&mut first);
    let mut second = block("0123456789abcdef0000000000000000");
    cipher.encrypt_block(&mut second);
    assert_eq!(&stream[..16], &first[..]);
    assert_eq!(&stream[16..], &second[..]);
}

/// Calls that end mid block must carry on from the same keystream byte, so
/// any split of a message encrypts exactly as one call over all of it.
#[test]
fn split_calls_are_one_continuous_keystream() {
    let key = block("2b7e151628aed2a6abf7158809cf4f3c");
    let iv = block("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let mut once: Vec<u8> = (0..600u32).map(|index| index as u8).collect();
    let mut split = once.clone();
    Ctr64Be::new(&key, &iv).apply(&mut once);
    let mut cipher = Ctr64Be::new(&key, &iv);
    let mut rest = split.as_mut_slice();
    for size in [1usize, 7, 15, 16, 17, 33, 509].iter().copied().cycle() {
        let take = size.min(rest.len());
        let (head, tail) = rest.split_at_mut(take);
        cipher.apply(head);
        rest = tail;
        if rest.is_empty() {
            break;
        }
    }
    assert_eq!(once, split);
}
