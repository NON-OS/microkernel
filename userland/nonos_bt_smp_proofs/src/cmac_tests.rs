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

//! AES-CMAC against RFC 4493, 4: the four examples under its one key.

use crate::hex::{arr, hex};
use nonos_bt_smp::cmac::aes_cmac;

const K: &str = "2b7e1516 28aed2a6 abf71588 09cf4f3c";
const M: &str = "6bc1bee2 2e409f96 e93d7e11 7393172a ae2d8a57 1e03ac9c 9eb76fac 45af8e51
                 30c81c46 a35ce411 e5fbc119 1a0a52ef f69f2445 df4f9b17 ad2b417b e66c3710";

#[test]
fn rfc4493_examples() {
    let (key, m) = (arr::<16>(K), hex(M));
    let cases = [
        (0, "bb1d6929 e9593728 7fa37d12 9b756746"),
        (16, "070a16b4 6b4d4144 f79bdd9d d04a287c"),
        (40, "dfa66747 de9ae630 30ca3261 1497c827"),
        (64, "51f0bebf 7e3b9d92 fc497417 79363cfe"),
    ];
    for (len, mac) in cases {
        assert_eq!(aes_cmac(&key, &[&m[..len]]), arr::<16>(mac), "length {len}");
    }
}

#[test]
fn parts_are_one_message() {
    let (key, m) = (arr::<16>(K), hex(M));
    let whole = aes_cmac(&key, &[&m[..40]]);
    assert_eq!(aes_cmac(&key, &[&m[..3], &m[3..17], &[], &m[17..40]]), whole);
    assert_ne!(aes_cmac(&key, &[&m[..39]]), whole);
}
