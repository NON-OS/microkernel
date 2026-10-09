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


//! SHA-384 and HMAC-SHA384 against FIPS 180-4, RFC 4231 and Python's
//! hashlib and hmac.

use super::expect::{hex, lines};
use crate::crypto::sha384::{hmac_sha384, sha384, Sha384};

#[test]
fn fips_180_4_abc() {
    assert_eq!(
        sha384(&[b"abc"]).to_vec(),
        hex("cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7")
    );
}

#[test]
fn rfc_4231_test_case_2() {
    assert_eq!(
        hmac_sha384(b"Jefe", &[b"what do ya want for nothing?"]).to_vec(),
        hex("af45d2e376484031617f78d2b58a6b1b9c7ef464f5a01b47e42ec3736322445e8e2240ca5e69e2c78b3239ecfab21649")
    );
}

#[test]
fn every_length_across_the_block_and_padding_edges() {
    let all = lines("sha384");
    assert_eq!(all.len(), 13);
    for f in all {
        let msg = hex(f[1]);
        assert_eq!(sha384(&[&msg]).to_vec(), hex(f[2]), "{} bytes", msg.len());
        // Fed a byte at a time, and read midway, it is the same hash.
        let mut h = Sha384::new();
        for (i, b) in msg.iter().enumerate() {
            h.update(core::slice::from_ref(b));
            if i == msg.len() / 2 {
                let _ = h.finish();
            }
        }
        assert_eq!(h.finish().to_vec(), hex(f[2]), "{} bytes, split", msg.len());
    }
}

#[test]
fn hmac_with_empty_short_block_and_longer_than_block_keys() {
    let all = lines("hmac384");
    assert_eq!(all.len(), 4);
    for f in all {
        let (key, msg) = (hex(f[1]), hex(f[2]));
        assert_eq!(hmac_sha384(&key, &[&msg]).to_vec(), hex(f[3]), "key of {} bytes", key.len());
        let (a, b) = msg.split_at(17);
        assert_eq!(hmac_sha384(&key, &[a, b]).to_vec(), hex(f[3]), "in two parts");
    }
}
