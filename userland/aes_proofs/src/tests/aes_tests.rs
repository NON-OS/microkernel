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

//! AES-128 against FIPS 197.

use crate::hex::hex;
use nonos_aes::Aes128;

pub(crate) fn block(text: &str) -> [u8; 16] {
    let mut out = [0u8; 16];
    out.copy_from_slice(&hex(text));
    out
}

// FIPS 197, appendix B.
#[test]
fn fips197_appendix_b() {
    let mut data = block("3243f6a8885a308d313198a2e0370734");
    Aes128::new(&block("2b7e151628aed2a6abf7158809cf4f3c")).encrypt_block(&mut data);
    assert_eq!(data.to_vec(), hex("3925841d02dc09fbdc118597196a0b32"));
}

// FIPS 197, appendix C.1.
#[test]
fn fips197_appendix_c1() {
    let mut data = block("00112233445566778899aabbccddeeff");
    Aes128::new(&block("000102030405060708090a0b0c0d0e0f")).encrypt_block(&mut data);
    assert_eq!(data.to_vec(), hex("69c4e0d86a7b0430d8cdb78070b4c55a"));
}
