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
//! The voucher with a contract deployed, as it will look once
//! nonos_nox_config names one: the path that signs, end to end but the key.

use super::typed_hash_tests::keccak;
use crate::server::typed::hashes::hashes;
use crate::server::typed::voucher::VOUCHER;

fn k(b: &[u8]) -> [u8; 32] {
    keccak(b).unwrap()
}

/// The three hashes, against EIP-712 written out by hand.
#[test]
fn the_hashes_are_eip712s() {
    let mut words = vec![0u8; 9 * 32];
    words[32 + 12..64].copy_from_slice(&[0x77; 20]);
    let h = hashes(&VOUCHER, 11_155_111, &[7; 20], &words, keccak).unwrap();
    let dom = [
        k(b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)")
            .as_slice(),
        &k(b"NONOS Settlement"),
        &k(b"1"),
        &[[0u8; 24].as_slice(), &11_155_111u64.to_be_bytes()].concat(),
        &[[0u8; 12].as_slice(), &[7; 20]].concat(),
    ]
    .concat();
    assert_eq!(h.domain_separator, k(&dom));
    let typehash = k(VOUCHER.typed.type_string.as_bytes());
    assert_eq!(h.struct_hash, k(&[typehash.as_slice(), &words].concat()));
    let digest = k(&[[0x19, 0x01].as_slice(), &h.domain_separator, &h.struct_hash].concat());
    assert_eq!(h.digest, digest);
}

#[test]
fn a_failed_hash_signs_nothing() {
    let words = vec![0u8; 9 * 32];
    assert!(hashes(&VOUCHER, 11_155_111, &[7; 20], &words, |_| None).is_none());
}
