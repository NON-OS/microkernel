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

use crate::crypto::asymmetric::rsa::{
    create_public_key, verify_pkcs1v15, verify_pkcs1v15_sha384, verify_pkcs1v15_sha512,
    RsaPublicKey,
};
use crate::crypto::hash::sha256;
use crate::crypto::hash::sha384::sha384;
use crate::crypto::hash::sha512::sha512;
extern crate alloc;
use alloc::vec::Vec;

fn di(total: u8, alg: u8, hlen: u8) -> [u8; 19] {
    [
        0x30, total, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, alg,
        0x05, 0x00, 0x04, hlen,
    ]
}
const MSG: &[u8] = b"nonos pkcs1 encoding";

fn identity_key(k: usize) -> RsaPublicKey {
    create_public_key(alloc::vec![0xFF; k], alloc::vec![0x01])
}

fn em(k: usize, between: &[u8], di: &[u8], hash: &[u8], tail: &[u8]) -> Vec<u8> {
    let body = between.len() + di.len() + hash.len() + tail.len();
    let mut out = alloc::vec![0x00, 0x01];
    out.resize(k - body - 1, 0xFF);
    out.push(0x00);
    for part in [between, di, hash, tail] {
        out.extend_from_slice(part);
    }
    out
}

#[test]
fn exact_encodings_verify_for_every_hash() {
    let key = identity_key(256);
    assert!(verify_pkcs1v15(&key, MSG, &em(256, &[], &di(0x31, 1, 32), &sha256(MSG), &[])));
    assert!(verify_pkcs1v15_sha384(&key, MSG, &em(256, &[], &di(0x41, 2, 48), &sha384(MSG), &[])));
    assert!(verify_pkcs1v15_sha512(&key, MSG, &em(256, &[], &di(0x51, 3, 64), &sha512(MSG), &[])));
    let tight = em(62, &[], &di(0x31, 1, 32), &sha256(MSG), &[]);
    assert!(verify_pkcs1v15(&identity_key(62), MSG, &tight));
}

#[test]
fn malformed_encodings_are_rejected() {
    let key = identity_key(256);
    let h = sha256(MSG);
    assert!(!verify_pkcs1v15(&key, MSG, &em(256, &[0x00], &di(0x31, 1, 32), &h, &[])));
    assert!(!verify_pkcs1v15(&key, MSG, &em(256, &[0xAB; 4], &di(0x31, 1, 32), &h, &[])));
    assert!(!verify_pkcs1v15(&key, MSG, &em(256, &[], &di(0x31, 1, 32), &h, &[0x00])));
    assert!(!verify_pkcs1v15(&key, MSG, &em(256, &[], &di(0x31, 1, 32), &sha256(b"other"), &[])));
    let h384 = sha384(MSG);
    assert!(!verify_pkcs1v15_sha384(&key, MSG, &em(256, &[0xAB; 19], &[], &h384, &[])));
    assert!(!verify_pkcs1v15_sha384(&key, MSG, &em(256, &[], &di(0x51, 3, 64), &h384, &[])));
    let short_ps = em(61, &[], &di(0x31, 1, 32), &h, &[]);
    assert!(!verify_pkcs1v15(&identity_key(61), MSG, &short_ps));
    let long_sig = [em(256, &[], &di(0x31, 1, 32), &h, &[]), alloc::vec![0x00]].concat();
    assert!(!verify_pkcs1v15(&key, MSG, &long_sig));
}
