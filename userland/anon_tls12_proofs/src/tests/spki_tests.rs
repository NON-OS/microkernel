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


//! Finding the key in a certificate, against the cryptography package's
//! reading of the same certificates, and never panicking on anything else.

use super::cases::{get, handshake};
use super::expect::{hex, lines};
use crate::tls12::server::certificate;
use crate::tls12::spki::spki;

fn leaf(name: &str) -> std::vec::Vec<u8> {
    let (_, body) = handshake(get(name).server).into_iter().find(|(k, _)| *k == 11).unwrap();
    certificate(&body).unwrap()
}

#[test]
fn the_key_is_the_one_the_certificate_holds() {
    let all = lines("spki");
    assert_eq!(all.len(), 3);
    for f in all {
        assert_eq!(spki(&leaf(f[1])).map(<[u8]>::to_vec), Some(hex(f[2])), "{}", f[1]);
    }
}

#[test]
fn every_truncation_and_every_flipped_length_is_handled() {
    let cert = leaf("chacha_certreq");
    for n in 0..cert.len() {
        assert_eq!(spki(&cert[..n]), None, "a certificate cut to {n} bytes");
    }
    for at in 0..cert.len().min(400) {
        let mut c = cert.clone();
        c[at] ^= 0x80;
        let _ = spki(&c);
    }
    assert_eq!(spki(&[0x30, 0x84, 0, 0, 0, 1, 0]), None, "four-byte lengths are refused");
    assert_eq!(spki(&[0x31, 0x00]), None);
}
