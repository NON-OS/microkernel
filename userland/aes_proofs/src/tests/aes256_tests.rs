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

//! AES-256 and its counter mode, against the published vectors.

use crate::hex::hex;
use nonos_aes::{Aes256, Ctr256Be};

fn key(text: &str) -> [u8; 32] {
    hex(text).try_into().expect("32 bytes")
}

fn block16(text: &str) -> [u8; 16] {
    hex(text).try_into().expect("16 bytes")
}

/// FIPS-197 appendix C.3.
#[test]
fn fips_197_c3() {
    let cipher = Aes256::new(&key("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"));
    let mut b = block16("00112233445566778899aabbccddeeff");
    cipher.encrypt_block(&mut b);
    assert_eq!(b.to_vec(), hex("8ea2b7ca516745bfeafc49904b496089"));
}

/// NIST SP 800-38A F.5.5, CTR-AES256.Encrypt, all four blocks in one call
/// and again split mid block, as a 509 byte cell leaves the keystream.
#[test]
fn sp800_38a_f55() {
    let k = key("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4");
    let iv = block16("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let plain = hex(concat!(
        "6bc1bee22e409f96e93d7e117393172a",
        "ae2d8a571e03ac9c9eb76fac45af8e51",
        "30c81c46a35ce411e5fbc1191a0a52ef",
        "f69f2445df4f9b17ad2b417be66c3710"
    ));
    let want = hex(concat!(
        "601ec313775789a5b7a7f504bbf3d228",
        "f443e3ca4d62b59aca84e990cacaf5c5",
        "2b0930daa23de94ce87017ba2d84988d",
        "dfc9c58db67aada613c2dd08457941a6"
    ));
    let mut whole = plain.clone();
    Ctr256Be::with_iv(&k, &iv).apply(&mut whole);
    assert_eq!(whole, want);

    let mut split = plain.clone();
    let mut stream = Ctr256Be::with_iv(&k, &iv);
    let (a, b) = split.split_at_mut(7);
    stream.apply(a);
    stream.apply(b);
    assert_eq!(split, want);
}

/// `new` is the zero IV that the virtual hop and INTRODUCE1 use.
#[test]
fn new_starts_at_a_zero_counter() {
    let k = key("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4");
    let mut a = vec![0u8; 40];
    let mut b = vec![0u8; 40];
    Ctr256Be::new(&k).apply(&mut a);
    Ctr256Be::with_iv(&k, &[0u8; 16]).apply(&mut b);
    assert_eq!(a, b);
    let mut first = [0u8; 16];
    Aes256::new(&k).encrypt_block(&mut first);
    assert_eq!(&a[..16], &first[..]);
}
